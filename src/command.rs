pub enum Command {
    Put { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Flush,
}
impl Command {
    pub fn from_args() -> Option<Self> {
        let mut args = std::env::args().skip(1); // skip binary name
        match args.next()?.as_str() {
            "put" => {
                let key = args.next()?;
                let value = args.next()?;
                Some(Self::Put { key, value })
            }
            "get" => {
                let key = args.next()?;
                Some(Self::Get { key })
            }
            "delete" => {
                let key = args.next()?;
                Some(Self::Delete { key })
            }
            "flush" => Some(Self::Flush),
            _ => None,
        }
    }
}
