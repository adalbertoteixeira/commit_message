# Helper for convention commits creation

Builds [convential commits](https://www.conventionalcommits.org/en/v1.0.0/) semi-automatically,
getting the JIRA Id from the branch name.

Pass a message and the binary will output the commit message the default value (`feat`), the current
branch number / JIRA ticket number and the message provided.
