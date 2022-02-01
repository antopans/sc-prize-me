elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait SecurityModule {

    #[endpoint(secDummy)]
    fn security_dummy(&self) -> SCResult<()> {
        
        Ok(())  
    }
}