use image::GenericImageView;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub dimensions: (u32, u32),
}

impl Texture {
    pub fn from_frame(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        dimensions: (u32, u32),
        label: Option<&str>,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &bytes,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            dimensions,
        }
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Self {
        let img = image::load_from_memory(bytes).expect("loaded from memory");
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Self {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        Self::from_frame(device, queue, &rgba, dimensions, label)
    }
}

pub struct VideoTexture {
    pub stream: crate::VideoStream,
    pub texture: Texture,
    last_update: std::time::Instant,
}

impl VideoTexture {
    pub fn new(
        path: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
    ) -> Self {
        let mut stream = crate::VideoStream::new(path);
        let frame = stream.get_next_frame().unwrap();
        let texture = Texture::from_frame(
            device,
            queue,
            frame.data(0),
            (frame.width(), frame.height()),
            label,
        );

        Self {
            stream,
            texture,
            last_update: std::time::Instant::now(),
        }
    }

    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) -> bool {
        if self.last_update.elapsed().as_millis() > self.stream.frame_time() {
            let frame = self.stream.get_next_frame().unwrap();
            self.texture = Texture::from_frame(
                device,
                queue,
                frame.data(0),
                (frame.width(), frame.height()),
                None,
            );
            self.last_update = std::time::Instant::now();
            return true;
        }
        false
    }
}
