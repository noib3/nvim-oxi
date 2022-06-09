use nvim_types::dictionary::Dictionary;

#[derive(Clone, Debug)]
pub struct CreateAugroupOpts {
    clear: bool,
}

impl From<CreateAugroupOpts> for Dictionary {
    fn from(opts: CreateAugroupOpts) -> Self {
        Self::from_iter([("clear", opts.clear)])
    }
}

impl<'a> From<&'a CreateAugroupOpts> for Dictionary {
    fn from(opts: &CreateAugroupOpts) -> Self {
        opts.clone().into()
    }
}
