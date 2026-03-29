## v. 2.4.0

- Pull Request Review Event Support:
    - Added support for the `pull_request_review` event trigger.
    - Distinct notifications for PR reviews based on their state: Approved (✅), Changes Requested (❌), and Commented (
      💬).
    - Included truncated review comments directly in the Telegram notification for quick context.
    - Added a new inline keyboard button "↗️ Link to Review" that routes the user directly to the specific
      comment/review anchor rather than the top of the PR.
- Updated `README.md` to include examples of the `pull_request_review` trigger.

## v. 2.3.0

- Proxy Server Support:
    - Added a new optional input `proxy_url`.
    - Supports routing requests through HTTP, HTTPS, and SOCKS5 proxies.
- Updated `action.yml` to include the new input.
- Updated `README.md`.

## v. 2.2 fix

- Fix `api_url` default value if not provided.

## v. 2.2

- Custom Bot API Server Support:
    - Added a new optional input `api_url`.
    - Telegram Docs: https://core.telegram.org/bots/api#using-a-local-bot-api-server
- Multi-platform Builds (ARM)
- Updated `README.md`
- Updated `Dockerfile`

## v. 2.1

- Added support for the `Workflow Dispatch` event. It displays information about a manually triggered workflow:
    - "Workflow Dispatched" or a custom title (input title);
    - "Workflow status" - the status of the triggered workflow;
    - "Actor" - the initiator of the workflow execution;
    - "Repository" - a link to the repository;
    - "Workflow" - the name of the triggered workflow;
    - "With custom inputs" - a list of custom input fields for the manual workflow trigger.
- Refactored text utility code into corresponding modules.
- Updated `README.md`