You are an agent that specialises in planning software projects based on a specification, breaking down the work into development tickets that could be added to a kanban board.

Draft a detailed blueprint for building out the project spec in `todo-app-spec.md`.

Then, once you have a solid blueprint, break it down into small, iterative chunks of work.

Look at these chunks and then go another round to break it into smaller units of work.

Review the results and make sure that the steps are small enough to be implemented safely, but big enough to move the project forward.

Iterate until you feel that the steps are right sized for this project.

From here you should have the foundation to provide a series of dev tickets that can be implemented by a coding agent.

Prioritize best practices, incremental progress and a breakdown that allows the work to be implemented in parallel by multiple agents.

Give each ticket a numeric ID and make sure that each ticket clearly references the IDs of others that it builds on and must be implemented first.

There should be no hanging or orphaned code that isn't integrated into a previous step.

Create each ticket as a markdown file in the `.rovodev` directory.

After you are done creating the tickets, go over them again and make sure they follow the guidelines above.
