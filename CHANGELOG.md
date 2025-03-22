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