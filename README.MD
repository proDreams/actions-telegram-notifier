# Napkin Tools: Telegram Notify Action

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/proDreams/actions-telegram-notifier/build-and-push-action.yml)
[![Код на салфетке](https://img.shields.io/badge/Telegram-Код_на_салфетке-blue)](https://t.me/press_any_button)
[![Заметки на салфетке](https://img.shields.io/badge/Telegram-Заметки_на_салфетке-blue)](https://t.me/writeanynotes)

A GitHub Action to send notifications to Telegram about the status of your GitHub Workflow runs. It helps you track
processes and get real-time updates on successful or failed executions.

[На русском](./README-RU.MD)

## Features

- Send status updates to Telegram (e.g., success, failure, cancelled).
- Customize notifications with a title, message, and footer.
- Include additional workflow details such as actor, repository, and workflow name.
- Automatically generates a commit link button in the notification, making it easier to track specific changes directly
  from Telegram.

## Prerequisites

You need:

- A Telegram bot token (you can create a bot using [BotFather](https://t.me/BotFather)).
- A chat ID to send messages to.

## Inputs

| Name            | Required | Description                                                                                                                                  |
|-----------------|----------|----------------------------------------------------------------------------------------------------------------------------------------------|
| `token`         | Yes      | Telegram Bot Token. Use GitHub Secrets to store this securely.                                                                               |
| `chat_id`       | Yes      | Chat ID where the message will be sent. Use GitHub Secrets to store this securely.                                                           |
| `thread_id`     | No       | Thread ID for group messages. Optional.                                                                                                      |
| `status`        | Yes      | Workflow status (e.g., `success`, `failure`, `cancelled`, `info`). Get from `${{ job.status }}`, or pass it yourself, for example `pending`. |
| `title`         | No       | Title text for the message. Defaults to a generic status message if not provided.                                                            |
| `message`       | No       | Custom message text to include in the notification.                                                                                          |
| `footer`        | No       | Footer text to include at the end of the notification.                                                                                       |
| `notify_fields` | No       | Comma-separated list of additional fields to include: `actor`, `repository`, `workflow`, `branch`, `commit`, `repo_with_tag`.                |

### Values of `notify_fields`

| Field           | Description                                                                |
|-----------------|----------------------------------------------------------------------------|
| `actor`         | The username of the person who triggered the workflow.                     |
| `repository`    | The name of the repository where the workflow is running.                  |
| `workflow`      | The name of the workflow triggered in GitHub Actions.                      |
| `branch`        | The branch in which the workflow was triggered.                            |
| `commit`        | The first line of the commit message.                                      |
| `repo_with_tag` | The repository name with the tag (if the workflow was triggered by a tag). |

## Usage

Here’s an example of how to use the action in your GitHub Workflow:

```yaml
name: CI
on:
  push:
    branches:
      - main

jobs:
  notify:
    runs-on: ubuntu-latest
    steps:
      - name: Run Telegram Notify Action
        uses: proDreams/actions-telegram-notifier@main
        if: always()
        with:
          token: ${{ secrets.TELEGRAM_BOT_TOKEN }}
          chat_id: ${{ secrets.TELEGRAM_CHAT_ID }}
          status: ${{ job.status }}
          notify_fields: "actor,repository,workflow"
```

### Advanced Usage

You can customize the notification by including additional inputs like `message` and `footer`:

```yaml
- name: Run Telegram Notify Action
  uses: proDreams/actions-telegram-notifier@main
  if: always()
  with:
    token: ${{ secrets.TELEGRAM_BOT_TOKEN }}
    chat_id: ${{ secrets.TELEGRAM_CHAT_ID }}
    status: ${{ job.status }}
    notify_fields: "actor,repository,workflow"
    message: "Custom message text"
    footer: "Footer text"
```

## Example Notifications

### Success Notification

![Success](./img/success.png)

## Error Handling

- If the Telegram API request fails, the action will log the error message and the Workflow will fail.
- Common issues include invalid `token` or `chat_id`.

## Development Details

- **Rust Version:** The action uses Rust 1.83, defined in the Dockerfile.
- **GitHub Actions Version Compatibility:** Compatible with any modern version of GitHub Actions.
- **Dockerfile:** The action is containerized, ensuring it runs consistently in any environment.

## Author

- Ivan "proDream" Ashikhmin

## Contributors

<a href="https://github.com/proDreams/actions-telegram-notifier/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=proDreams/actions-telegram-notifier" />
</a>
