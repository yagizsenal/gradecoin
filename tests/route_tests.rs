#[cfg(test)]
mod tests {
    use gradecoin::schema::{Block, Db, InitialAuthRequest, MetuId, Transaction, User};
    use pretty_assertions::assert_eq;

    use gradecoin::routes::consensus_routes;
    use warp::http::StatusCode;

    /// Create a mock database to be used in tests
    fn mocked_db() -> Db {
        let db = Db::new();

        db.users.write().insert(
            "fingerprint_of_some_guy".to_owned(),
            User {
                user_id: MetuId::new("e254275".to_owned(), "DtNX1qk4YF4saRH".to_owned()).unwrap(),
                public_key: "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA5yWTMeFqr2nvOC9oR5Wq
/nzcNlwCIaziojt7rJ4BBvuwkT0tERDz8AgvUsaewiB+Fz5OXTeb3WAB1FEXnBXG
ekrGzvC8jHQMKHyNoWzUlpQJ9UMtdQIWPOCuMyLpc+rNPL3428U8UpldjbTHHyq2
/ef6abkdj+XWg/slYtrFeOf3ktc1l50R4k8VO8L6kQuh2+YIjXGPLShRaqnUQPtH
8LFPX4bO9lJ9mAoMZFec6XVwumn/uqu9jyWQL6qh6gtwQHgN+A9wGvzVvltJ9h8s
shSHWWtBD0M19ilbXhKyBsHSSZkpx+TAvFhfQ8JURw7KqahUPVlCwJ5OIKccJ/6F
FQIDAQAB
-----END PUBLIC KEY-----"
                    .to_owned(),
                balance: 30,
                is_bot: false,
            },
        );

        db.users.write().insert(
            "fingerprint_of_foo".to_owned(),
            User {
                user_id: MetuId::new("e223715".to_owned(), "1H5QuOYI1b2r9ET".to_owned()).unwrap(),
                public_key: "NOT_USED_FOR_THIS_USER".to_owned(),
                balance: 0,
                is_bot: false,
            },
        );

        /*
        -----BEGIN RSA PRIVATE KEY-----
        MIIEpAIBAAKCAQEA5yWTMeFqr2nvOC9oR5Wq/nzcNlwCIaziojt7rJ4BBvuwkT0t
        ERDz8AgvUsaewiB+Fz5OXTeb3WAB1FEXnBXGekrGzvC8jHQMKHyNoWzUlpQJ9UMt
        dQIWPOCuMyLpc+rNPL3428U8UpldjbTHHyq2/ef6abkdj+XWg/slYtrFeOf3ktc1
        l50R4k8VO8L6kQuh2+YIjXGPLShRaqnUQPtH8LFPX4bO9lJ9mAoMZFec6XVwumn/
        uqu9jyWQL6qh6gtwQHgN+A9wGvzVvltJ9h8sshSHWWtBD0M19ilbXhKyBsHSSZkp
        x+TAvFhfQ8JURw7KqahUPVlCwJ5OIKccJ/6FFQIDAQABAoIBADTZGnZlG4dPqSon
        bKgxSA83bQHgt3wLkyWUhApLdeCq2wvZ+NvWDG/s7yT11IZ991ZJIJGfjTtoIALz
        J3rAX8jGH/5gfDuArOb000z9HP3wivZQjawa9gqlNC7s5INkQ9iHdsaIqeoYtpMX
        qg8uLPiQeWiCsoeb/Rff7ARWEKA7udoZ2uZcZFMHTKx+mBpk8IiepQAJPBRVwmXk
        x/3LTaezi6Tkvp/k/gf4IeSICiRGFRmm2Vxciduj11/CrdTHPQLz/Rh5/IN8Bkry
        xdQdQxxhwxF/ap6OJIJyguq7gximn2uK0jbHY3nRmrF8SsEtIT+Gd7I46L/goR8c
        jQOQRmECgYEA9RJSOBUkZMLoUcC2LGJBZOAnJZ7WToCVdu3LrPceRYtQHwcznW4O
        NAHF+blQRzqvbMi11ap8NVpkDDu0ki/Yi2VdSVjQmlaOcpAXjN6T5ZrKoz61xj4g
        2T2/K6d6ypkZRKPhKCC1iI419rq/APVEZHYCl7jZp4iD2izHiegZYccCgYEA8XRK
        rfVuPiYsaB07eJrRKKjuoM1Jcr19jZyXY8sbALRcExaTX2CRaPA7binVeDBXayQ1
        I0+kA1nV1EI+ROegV+b6gs2YaUmMJzI1yLqMqGDgHFxFvhkDsZaI+/V+G9eOLEt4
        5ic5tImfZITLE/GSC8b+C16gxMGUN4t9gHq2okMCgYAKyNedaDDFzl3y2wwpP9mo
        2sReP3Mm2Tm6lhRUdDt8y/impOZ8kw9E8p8HskP6HncBzoNR98KnhmbIswfrNvfM
        ipVkWOg1IoH6QKUIqfLQM9OfA290Xd+ML89t2Fzq9XnLL3sFDQtwCvIM/YLSQ/jS
        gu7yRkwttzA2NapCQ1h6mQKBgQClwBwn8Qyd01y2mCKkNzsP+2/cqTAbeSNAXFe8
        pMfDowx1+hBu7/7CF+/kPwmQuTa5kSB9PgWsWzYjwNm4OX1j+mbL9lEDLf7tRVWQ
        lydJyz7tmRYzWj6j4V/l/u90M3QgyiqTbCf73GG0AkjaRwHn3dG1gl9A0lZqDvK3
        iQXouwKBgQCrx6SCnEkhLISSZpzdDehtWmyCQJIwcdlRQlAmFLVn+TJHTXR7xUm2
        VpTrPTfaYWx83OQUn/OZqY5gIQ+jlfwqnVg+PDQQ/P09/4xygRCLvjL6NCSvtkj1
        MRArEl4y68+jZLRu74TVG0lXi6ht6KhNHF6GiWKU9FHZ4B+btLicsg==
        -----END RSA PRIVATE KEY-----
        */

        db.pending_transactions.write().insert(
            "fingerprint_of_foo".to_owned(),
            Transaction {
                source: "fingerprint_of_foo".to_owned(),
                target: "fingerprint_of_foo".to_owned(),
                amount: 2,
                timestamp: chrono::NaiveDate::from_ymd(2021, 04, 13).and_hms(20, 55, 30),
            },
        );

        *db.blockchain.write() = Block {
            transaction_list: vec![
                "foo_public_key_signature".to_owned(),
                "bar_public_key_signature".to_owned(),
                "baz_public_key_signature".to_owned(),
            ],
            nonce: 6920405,
            timestamp: chrono::NaiveDate::from_ymd(2021, 04, 13).and_hms(20, 55, 00),
            hash: "0000009745f2f09c968c095af75e8ab87eba9be90a93e5df464f83ea7ec08537".to_owned(),
        };

        db
    }

    /// Test simple GET request to /transaction, an endpoint that exists
    /// https://tools.ietf.org/html/rfc7231#section-6.3.1
    /// We should get the only pending transaction available in the database as json
    #[tokio::test]
    async fn get_pending_transactions() {
        let db = mocked_db();

        let reply = consensus_routes(db);

        let res = warp::test::request()
            .method("GET")
            .path("/transaction")
            .reply(&reply)
            .await;

        assert_eq!(res.status(), StatusCode::OK);

        let expected_json_body = r#"{"fingerprint_of_foo":{"source":"fingerprint_of_foo","target":"fingerprint_of_foo","amount":2,"timestamp":"2021-04-13T20:55:30"}}"#;

        assert_eq!(res.body(), expected_json_body);
    }

    /// Test simple GET request to /block, an enpoint that exists
    ///
    /// https://tools.ietf.org/html/rfc7231#section-6.3.1
    ///
    /// Should return the single block available in the database as json
    #[tokio::test]
    async fn get_blockchain() {
        let db = mocked_db();
        let filter = consensus_routes(db);

        let res = warp::test::request()
            .method("GET")
            .path("/block")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::OK);

        let expected_json_body = r#"{"transaction_list":["foo_public_key_signature","bar_public_key_signature","baz_public_key_signature"],"nonce":6920405,"timestamp":"2021-04-13T20:55:00","hash":"0000009745f2f09c968c095af75e8ab87eba9be90a93e5df464f83ea7ec08537"}"#;
        assert_eq!(res.body(), expected_json_body);
    }

    /// Test a simple GET request to a nonexisting path
    /// https://tools.ietf.org/html/rfc7231#section-6.5.4
    /// Should respond with 404 and stop
    #[tokio::test]
    async fn get_nonexisting_path_404() {
        let db = mocked_db();
        let filter = consensus_routes(db);

        let res = warp::test::request()
            .method("GET")
            .path("/this_path_does_not_exist")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    /// Test a POST request to /transaction, an endpoint that exists
    ///
    /// https://tools.ietf.org/html/rfc7231#section-6.3.2
    ///
    /// Should accept the json request, create
    /// the transaction and add it to pending transactions in the db
    #[tokio::test]
    async fn post_auth_json_201() {
        let db = mocked_db();
        let filter = consensus_routes(db.clone());

        let res = warp::test::request()
            .method("POST")
            .json(&Transaction {
            source: "fingerprint_of_some_guy".to_owned(),
            target: "31415926535897932384626433832795028841971693993751058209749445923".to_owned(),
            amount: 2,
            timestamp: chrono::NaiveDate::from_ymd(2021, 04, 13).and_hms(20, 55, 30),
        })
            .header("Authorization", "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ0aGEiOiJiODI4ZmYwNzM5NjFmMDA2YjU3ZmNkZWMwNmEwZTY1YSIsImV4cCI6MjAwMDAwMDAwMCwiaWF0IjoyNTE2MjM5MDIyfQ.Zwr2BPYzxvdAy8ZjCWSA3dp96KKVHjRDu9imHKCJ0NUeD5fM4D678B6pRZk9ev8PvbEI88MgYVW9akj_IsFwrsho7Tpsh7ym593ZOMwtSWSxxzxuKuGhAuKHDiQlXMFfv6kl8-eXIxa7xbV_0m81vAz6_vYQnVnlhjRQ26LogJRixWpUKV7-mPdRHhrK6dUxi9CwVuc9fdYhd6n2FMKT2AlAnk472fDa3G1oPsFLkp56eJ6_rKWrALHFWItfNvY4zFvwXXyNjMu-0EYJiaQIXFVNnipsy5Sy2HckdY3UiYS2lFUwHIczNrdrLM7NQhJQmbJLD6XRAMifH6bA1ZxH8A")
            .path("/transaction")
            .reply(&filter)
            .await;

        println!("{:?}", res.body());
        assert_eq!(res.status(), StatusCode::CREATED);
        for i in db.pending_transactions.read().iter() {
            println!("{:?}", i);
        }
        assert_eq!(db.pending_transactions.read().len(), 2);
    }

    /// Test a POST request to /transaction, an endpoint that exists with an incorrect JWT in the
    /// Authorization header
    ///
    /// https://tools.ietf.org/html/rfc7231#section-6.3.2
    ///
    /// Should reject the request
    #[tokio::test]
    async fn post_auth_json_400() {
        let db = mocked_db();
        let filter = consensus_routes(db.clone());

        let res = warp::test::request()
            .method("POST")
            .json(&Transaction {
                source: "some_fingerprint".to_owned(),
                target: "some_other_fingerprint".to_owned(),
                amount: 2,
                timestamp: chrono::NaiveDate::from_ymd(2021, 04, 09).and_hms(14, 30, 00),
            })
            .header(
                "Authorization",
                "Bearer aaaaaaaasdlkjaldkasljdaskjlaaaaaaaaaaaaaa",
            )
            .path("/transaction")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(db.pending_transactions.read().len(), 1);
    }

    /// Test a POST request to /block, an endpoint that exists
    ///
    /// https://tools.ietf.org/html/rfc7231#section-6.3.2
    ///
    /// Should accept the json request, create
    /// the block
    #[tokio::test]
    async fn post_block_auth_201() {
        let db = mocked_db();
        let filter = consensus_routes(db.clone());

        db.pending_transactions.write().insert(
            "fingerprint_of_some_guy".to_owned(),
            Transaction {
                source: "fingerprint_of_some_guy".to_owned(),
                target: "31415926535897932384626433832795028841971693993751058209749445923"
                    .to_owned(),
                amount: 2,
                timestamp: chrono::NaiveDate::from_ymd(2021, 04, 13).and_hms(20, 55, 30),
            },
        );

        let res = warp::test::request()
            .method("POST")
            .json(&Block {
                transaction_list: vec!["fingerprint_of_some_guy".to_owned()],
                nonce: 3222170950,
                timestamp: chrono::NaiveDate::from_ymd(2021, 04, 13).and_hms(23, 38, 00),
                hash: "0000002149b72e0b348c32ac442a50ced4efbd1df7d48b377733d55dfe4f3577".to_owned(),
            } )
        .header("Authorization", "Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ0aGEiOiIwMDAwMDAyMTQ5YjcyZTBiMzQ4YzMyYWM0NDJhNTBjZWQ0ZWZiZDFkZjdkNDhiMzc3NzMzZDU1ZGZlNGYzNTc3IiwiZXhwIjoxMDAwMDAwMDAwMCwiaWF0IjoxNTE2MjM5MDAyMn0.2hZkIZiOJ9DpHwzL_rTCHGkTxs0S_-KD58a2bXuICogjrW6Z3o9LaJ95d8kpXSrAUCaFWN_-TNbk26LczDFSAhtTxpPh6ELXyLN82wAOCjsp8qLgXJQCnqNY7VFZdRUW0HDwHBLY2PVH40wprPkF7mLLonUPKcJUg9TLnmqHGriSECgv2-XZt2mD9XdTkofqIR-JRR3qoErb4G4xemMNYlWMuJK8I66Vj8kNytSt4wSCzkrWfmk5yyi7q1N3beo4nkP0DKzVstXZzWjn_qMZNZulfOdrNiKwHpwqX_bL66LB0rVpKcI5f6N6w5GZBKsdeUly7dCtKbxwYX3mV-7I_g")
            .path("/block")
            .reply(&filter)
            .await;

        println!("RESPONSE: {:?}", res.body());

        // should be reflectled on the db as well
        assert_eq!(
            db.blockchain.read().hash,
            "0000002149b72e0b348c32ac442a50ced4efbd1df7d48b377733d55dfe4f3577".to_owned()
        );
        assert_eq!(res.status(), StatusCode::CREATED);
    }

    /// Test a POST request to /block, an endpoint that exists
    ///
    /// https://tools.ietf.org/html/rfc7231#section-6.3.2
    ///
    /// Should reject the block because there aren't enough zeroes in the hash
    #[tokio::test]
    async fn post_block_wrong_hash() {
        let db = mocked_db();
        let filter = consensus_routes(db.clone());

        let res = warp::test::request()
            .method("POST")
            .header("Authorization", "Bearer foo.bar.baz")
            .json(&Block {
                transaction_list: vec!["foobarbaz".to_owned(), "dazsaz".to_owned()],
                nonce: 1000, // not valid
                timestamp: chrono::NaiveDate::from_ymd(2021, 04, 12).and_hms(05, 29, 30),
                hash: "tnarstnarsuthnarsthlarjstk".to_owned(),
            })
            .path("/block")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    /// Test a POST request to /block, an endpoint that exists
    ///
    /// https://tools.ietf.org/html/rfc7231#section-6.3.2
    ///
    /// Should reject the block because transaction list is empty
    #[tokio::test]
    async fn post_block_with_empty_transaction_list() {
        let db = mocked_db();
        let filter = consensus_routes(db.clone());

        let res = warp::test::request()
            .method("POST")
            .header("Authorization", "Bearer foo.bar.baz")
            .json(&Block {
                transaction_list: vec![],
                nonce: 1000, // not valid
                timestamp: chrono::NaiveDate::from_ymd(2021, 04, 12).and_hms(05, 29, 30),
                hash: "thisisnotavalidhash".to_owned(),
            })
            .path("/block")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    /// Test a POST request to /block, an endpoint that exists
    ///
    /// https://tools.ietf.org/html/rfc7231#section-6.3.2
    ///
    /// Should reject the block because hash has enough zeroes but is not the actual hash of the
    /// block
    #[tokio::test]
    async fn post_block_incorrect_hash() {
        let db = mocked_db();
        let filter = consensus_routes(db.clone());

        let res = warp::test::request()
            .method("POST")
            .json(&Block {
                transaction_list: vec![],
                nonce: 12314,
                timestamp: chrono::NaiveDate::from_ymd(2021, 04, 13).and_hms(20, 55, 00),
                hash: "0000001111111111111111111111111111111111111111111111111111111111".to_owned(),
            })
            .path("/block")
            .reply(&filter)
            .await;

        println!("{:?}", res.body());
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            db.blockchain.read().hash,
            "0000009745f2f09c968c095af75e8ab87eba9be90a93e5df464f83ea7ec08537"
        );
    }

    /// Test a POST request to /register, an endpoint that exists
    ///
    /// https://tools.ietf.org/html/rfc7231#section-6.3.2
    ///
    /// Should accept the json request, create a new user and
    /// add it to the user hashmap in the db
    // #[tokio::test]
    // async fn post_register_priviliged_user() {
    //     let db = mocked_db();
    //     let filter = consensus_routes(db.clone());

    //     let res = warp::test::request()
    //         .method("POST")
    //         .json(&priviliged_mocked_user())
    //         .path("/register")
    //         .reply(&filter)
    //         .await;

    //     println!("{:?}", res.body());
    //     assert_eq!(res.status(), StatusCode::CREATED);
    //     assert_eq!(db.users.read().len(), 2);
    // }

    /// Test a POST request to /transaction, an endpoint that exists
    /// https://tools.ietf.org/html/rfc7231#section-6.3.2
    /// Should NOT accept the json request as the user is unpriviliged
    // #[tokio::test]
    // async fn post_register_unpriviliged_user() {
    //     let db = mocked_db();
    //     let filter = consensus_routes(db.clone());

    //     let res = warp::test::request()
    //         .method("POST")
    //         .json(&unpriviliged_mocked_user())
    //         .path("/register")
    //         .reply(&filter)
    //         .await;

    //     println!("{:?}", res.body());
    //     assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    //     assert_eq!(db.users.read().len(), 1);
    // }

    /// Test a POST request to /transaction, an endpoint that exists with a longer than expected
    /// payload
    ///
    /// https://tools.ietf.org/html/rfc7231#section-6.5.11
    ///
    /// Should return 413 to user
    #[tokio::test]
    async fn post_too_long_content_413() {
        let db = mocked_db();
        let filter = consensus_routes(db);

        let res = warp::test::request()
            .method("POST")
            .header("content-length", 1024 * 36)
            .path("/transaction")
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    /// Test the User Authentication Process
    #[tokio::test]
    async fn user_authentication() {
        let db = mocked_db();
        let filter = consensus_routes(db);

        let res = warp::test::request()
            .method("POST")
            .json(&InitialAuthRequest {
                c: "D9OKSp4XD+niltqhoiTEyz3pTxGm5ZKYVNFPofW40M6Km7wE7FgIpfTkurBZ6tQsG/rYPRsd6C/Qo+o3HrgOYC8BDprwpnYb7UnJdL2pe44ZMEsPAmDAdwTP9WozY0lr+bjEjtTM1mVQnIdfknychFek/FNi3l8MrapeFTxFaTMGxWuS1+wEuAkcz4AR4+jooaXVAEpKrPiSXqbywF9OQ41tk0kRiXn234dj40ndND+GlfMgghITuBJrJx6tzLppAZNIIGwUjQDt5Oib5dEGrPOe+rran1D26YNhZOtrfYEGyUSN+/58HbItQlLrgFhL6zRT7ojw/Eg4jYXndK0xNgYGyhAn5UI/qnI2NPpZU7Wd3sJKlWc7HfrjNnKVKlcrhHtYy3FXfN/hLg7SFmuSfXqqvVbNVT6pEDU6Y5NahOYaE/vkL0no7F7lz0UjAlgQCmn5yN7mKs3yLSnlx6hmsK/fVoqGBcOIbYY5gzYMlAQ3E+lq0p2MPEoWC8NYxStSeo9M8uLYT6Jl3hYVf8aLgd1l0HEiCyT+kWxvcR5hw42I7gqaoUcnr53Zm1mYK30/fvZ6lxsrb4FphldgQC5fx6nwEgjaLUeB4n0oZTSRLbrd9ZXCjUG4FNmM+sOklhIXyTYUj4VcBSwZuAvJZEFf2em68e7ySJs/ysz+TGu3eVeRc+voAvI9mGLxWnSEjWx64po7PO61uG6ikadHZH+wIw==".to_owned(),
                iv: "bmV2ZXJtaW5kdGhlbmZ1aw==".to_owned(),
                key: "Xd6/VSuFKqayNHspcFJSm+PAHNoTmcR4SsMijSyuyEh6PS5rdvO4W98AhxW4VBrRO1ljfEMeFq835NEDame511D2pim00Xv0HPIYSDW6pIJA1hy+Np/WyC7PCxvKy0hPzTmHMpFmM+aF43BknJdYlPUhY4cww/xScU6WxuKIsEQNORRhQds8CHOO0EGcOjHVvR2xqnOda1g/rI7mfNMATHj9ZRsB9GH6QG5WTUbo9/71cDAILF+28TG40jSKvY2KzO9vr668tgqoMV2vLnXQa1AD9ZWmdHHdjiXuiH3X0uXxHrfjH7HeXi/HOj/pgCX12jKsEsRwkBTGL4koObH6pQ==".to_owned(),
            })
            .path("/register")
            .reply(&filter)
            .await;

        println!("{:?}", res);
        assert_eq!(res.status(), StatusCode::CREATED);
    }
}

// TODO: POST block without correct transactions test <09-04-21, yigit> //
// TODO: POST transaction while that source has pending transaction test <09-04-21, yigit> //
