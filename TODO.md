# TODO

## Tests
- [ ] Route Tests
    - [ ] Malformed JSON bodies
    - [ ] Valid JSON with missing fields
    - [ ] Valid JSON with extra fields

## Someday, Maybe?
- [ ] CHAOS MODE, 3 different coins, combine them to make 1 gradecoin
- [X] NICE MODE, transactions move the amount, then award some to the sender, everyone starts with 10 gradecoins
- [ ] [SQLite](https://unixsheikh.com/articles/sqlite-the-only-database-you-will-ever-need-in-most-cases.html), not PostgreSQL

# Done & Brag
- [x] Switch to RwLock (parking_lot) (done at 2021-04-07 03:43, two possible schemes to represent inner Db (ledger) in code)
- [x] We need our own representation of students and their grades, "there is no blockchain" (done at 2021-04-12 00:05)
- [x] pick a block proposal scheme (= pick hash function) [list of hash functions](https://en.bitcoinwiki.org/wiki/List_of_hash_functions) (done at 2021-04-12 05:30)
- [x] check the nonce for incoming blocks (done at 2021-04-12 05:30)
----
- [X] pick a user authentication scheme = [JWT](https://tools.ietf.org/html/rfc7519) Seems perfect
- [X] implement JWT
- [X] users should be able to _sign_ their transactions
----
- [x] Verbose error messages (use error.rs from [logrocket](https://blog.logrocket.com/create-an-async-crud-web-service-in-rust-with-warp/) ❓) (done at 2021-04-13 20:39, not happy with the result)
- [x] Transactions should be rejected if the user cannot afford to send the amount
- [X] Schema Tests
- [x] /register is currently accepting non-encrypted (regular JSON) payloads (2021-04-14 19:19)
- [x] /register should check for public key pem format and assign signatures
----
- [x] Recover database from files
- [.] POST requests to /block should be authenticated as well (2021-04-13 04:50, they now are but until we make error messages **Verbose** there's not much point in testing because I honestly cannot trace the code)
- [X] Blocks should "play out" the transactions and execute transactions (2021-04-14 21:29)
- [X] "Coinbase" ("by" of the first transaction of the block) should get rewarded for their efforts (2021-04-14 21:48)
- [X] Implemented Bank Account (2021-04-14 23:28)
- [x] use [juice](https://www.getzola.org/themes/juice/) theme ~~with [template rendering](https://blog.logrocket.com/template-rendering-in-rust/)~~ zola to create a landing page. (done at 2021-04-15 03:41, in the most hilarious way possible)
----
- [x] IV is array, should be hex, implement checks
- [x] Restore students from disk
----
- [x] User Authentication/Authentication Tests (2021-04-16 15:34, Thanks Austin, Tx)
