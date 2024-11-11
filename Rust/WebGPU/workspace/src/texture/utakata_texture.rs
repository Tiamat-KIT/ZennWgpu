pub struct UtakataTexture {
    pub diffuse_texture: wgpu::Texture,
    pub diffuse_rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    pub dimensions: (u32,u32),
    pub texture_size: wgpu::Extent3d
}
impl UtakataTexture { 
    pub fn new(device: &wgpu::Device) -> Self{
        let diffuse_bytes = include_bytes!("../img/utakata.jpeg");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();

        use image::GenericImageView;
        let dimensions = diffuse_image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1
        };

        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
                view_formats: &[]
            }
        );

        Self {
            diffuse_texture,
            diffuse_rgba,
            dimensions,
            texture_size
        }

    }

    pub fn texture_write_queue(&self,queue: &wgpu::Queue) {
        queue.write_texture(
            wgpu::ImageCopyTexture { 
                texture: &self.diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All
            }, 
            &self.diffuse_rgba,
            wgpu::ImageDataLayout { 
                offset: 0,
                bytes_per_row: Some(4 * self.dimensions.0),
                rows_per_image: Some(self.dimensions.1)
            },
            self.texture_size
        );
    }

    pub fn texture_bind_group(&self,device: &wgpu::Device) -> (wgpu::BindGroupLayout,wgpu::BindGroup) {
        let diffuse_texture_view = self.diffuse_texture.create_view(
            &wgpu::TextureViewDescriptor::default()
        );
        let diffuse_sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("Texture Sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        let texture_bindgroup_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            }
        );

        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("texture_bind_group"),
                layout: &texture_bindgroup_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &diffuse_texture_view
                        )
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(
                            &diffuse_sampler
                        )
                    }
                ]
            }
        );
        return (texture_bindgroup_layout,diffuse_bind_group)
    }
}