transaction-manager is a simple utility to read a list of transactions and output the state of client accounts after all transactions are applied.

## Example Usage

Will output account list to stdout:
<pre>
cargo run resources/transaction-list.csv
</pre>

Will output account list to csv file:
<pre>
cargo run resources/transaction-list.csv > resources/account-list.csv
</pre>

## License

This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/tjhaskel/transaction-manager/blob/master/LICENSE.md) file for details

## Notes

* The logic in the client module regarding Dispute, Resolve, and Chargeback transactions assumes that the previous transaction being referenced is only of type "Deposit". There may be unexpected behaviour if a "Withdrawal" transaction is referenced instead.
* Other assumptions I'm making about transaction "rules" are specified in the transaction_error module and enforced in the client module.
* Because the csv reader processes one transaction at a time, transaction_manager could be modified to accept, verify, and merge concurrent streams of input data.
* The tests currently written are meant to cover only the most important functionality, and do not represent complete unit test coverage. If this were a real project, I would add tests to cover all possible results from all functions, including errors.
