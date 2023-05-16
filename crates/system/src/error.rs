pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, thiserror::Error, Debug)]
pub enum Error {
    #[error("SDL2 error ocurred: {0}")]
    Sdl(#[from] sdl2::Error),
    #[error("An error ocurred initializing SDL2: {0}")]
    SdlInit(String),
    #[error("Failed to retrieve SDL2 Event Pump: {0}")]
    EventPumpInit(String),
    #[error("Window Build Error: {0}")]
    WindowBuildError(#[from] sdl2::video::WindowBuildError),
    /// An error from SDL2 and C bindings, built from `IntegerOrSdlError`.
    /// Described by `sdl2` as follows:
    ///
    /// `A given integer was so big that its representation as a C integer would be negative.`
    #[error("Window Build Error: {0}")]
    IntegerOverflowError(#[from] sdl2::IntegerOrSdlError),
    #[error("Failed to initialize Video Subsystem: {0}")]
    VideoSubsystemInitialization(String),
    #[error("Failed to render window: {0}")]
    WindowRenderError(String),
    #[error("Emulator memory is out of bounds. Max: {0} Reached: {1}")]
    EmulatorMemoryOutOfBounds(usize, usize),
}
