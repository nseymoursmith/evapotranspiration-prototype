use crate::main;

#[cfg(test)]
mod test_sense_check_route {
    #[test]
    fn assert_response() {
        let res = warp::test::request()
            .path("/1/2")
            .reply(&main);


        println!("{:?}", res.await.body());

        assert_eq!(2 + 2, 5);
    }
}
