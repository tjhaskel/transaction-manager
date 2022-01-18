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
