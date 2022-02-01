elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ParameterModule {

    #[endpoint(paramDummy)]
    fn parameter_dummy(&self) -> SCResult<()> {
        
        Ok(())  
    }
}