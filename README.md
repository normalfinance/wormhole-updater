## Queries Verification on Solana PoC

This is a demo of verifying and parsing [Wormhole Queries](https://wormhole.com/queries/) on Solana.

This project was made with [Anchor](https://www.anchor-lang.com/).

Learn more about developing with Queries in [the docs](https://docs.wormhole.com/wormhole/queries/getting-started).

> N.B. This is a work-in-progress provided for example purposes only.

- [x] Verify mainnet queries using an active mainnet core bridge guardian set account
- [ ] Verify testnet queries using an active testnet core bridge guardian set account
- [ ] Verify mocked queries using a mock core bridge guardian set account on a mainnet address
- [ ] Allow for cleanup of signature set accounts
- [x] Validate a query result passed via instruction data
- [ ] Validate a query result passed via account
- [ ] Rust parsing for all query requests and responses

## Tests

To run the tests, `anchor test`.

---

⚠ **This software is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
implied. See the License for the specific language governing permissions and limitations under the License.**
