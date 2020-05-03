


#[cfg(test)]
mod tests {
    struct Row {
        id: u32,
        username: [u8; 32],
        email: [u8; 255]
    }
    fn serialize_data(source: Row, table: Table) {

    }
    #[test]
    fn test_serializer() {

        let id = "1".as_bytes();
        let username  = b"john.smith";
        let email = b"test@email.com";

    }
}
