use bytes::Bytes;

pub struct DoBlockRequest {
    /// The header of the block
    header: Bytes,
    /// The transactions that make up the block
    transactions: Vec<Bytes>,
}
pub struct DoBlockResponse {
    /// The new state root
    state_root: Bytes,
}

pub fn do_block(do_block_request: DoBlockRequest) -> DoBlockResponse {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // TODO
        assert_eq!(true, true);
    }
}
