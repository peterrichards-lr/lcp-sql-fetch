---

   Gemini Prompt: Implement LCP SQL Fetch

   Goal: Transform this template into lcp-sql-fetch, a tool that executes local SQL scripts on a Liferay Cloud database via lcp shell (using expect for automation) and then
   downloads the results using lcp files.

   Instructions for Gemini:
   Follow these steps to implement the core logic, leveraging the patterns from the lcp-db-reset tool and the provided Liferay Cloud API snippets.

---

1. Update CLI Structure (src/cli.rs)
   Define a fetch subcommand with the following arguments:
   - project (-p): Project ID or full ID.
   - environment (-e): Optional environment suffix.
   - service (-s): Default to liferay.
   - file (-f): Path to the local .sql script.
   - output (-o): Local path for the result file (default: output.txt).
   - password (-P): Database password (prompted securely if missing).

2. Implement API Utilities (src/core/)
   Integrate the following logic based on the user's provided snippets:
   - Auth: Check for a local lcp token or implement a fallback that uses https://api.liferay.cloud/login if credentials are provided via environment variables.
   - Project Discovery: Optionally use the GraphQL endpoint https://api.liferay.cloud/graphql with the ProjectsQuery to validate project existence before attempting a shell.

3. Implement the "Run & Fetch" Flow (src/main.rs)
   The fetch command should execute this exact lifecycle:
   1. Project ID Resolution: Use the "Smart Detection" logic (detect hyphen or prompt for environment).
   2. Instance Discovery: Use the lcp shell table-parsing method to find the first Running instance.
   3. File Upload: Execute lcp files upload -p <ID> -s <service> <local_sql> /tmp/query.sql.
   4. Automated Execution (Expect):
      - Generate an expect script to shell into the instance.
      - Execute: psql -U dxpcloud -d lportal -f /tmp/query.sql > /tmp/output.txt.
      - Exit the shell.
   5. File Download: Execute lcp files download -p <ID> -s <service> /tmp/output.txt <local_output>.
   6. Cleanup: Run a final lcp exec or expect block to rm /tmp/query.sql /tmp/output.txt inside the container.

4. Safety & Verification
   - Ensure expect availability is checked at runtime.
   - Use tempfile for the expect script.
   - Add a progress indicator (using indicatif from the template) for the upload/download phases.

---
