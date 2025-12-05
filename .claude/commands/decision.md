---
description: Manage decision graph - track algorithm choices and reasoning
allowed-tools: Bash(losselot:*, make:*)
argument-hint: <action> [args...]
---

# Decision Graph Management

You are helping manage the losselot decision graph - a DAG that tracks algorithm decisions, comparisons, and rationale for audio analysis improvements.

## Available Actions

Based on $ARGUMENTS, perform the appropriate action:

### List/View Commands
- `nodes` or `list` - Show all decision nodes: `losselot db nodes`
- `edges` - Show all edges: `losselot db edges`
- `graph` - Show full graph as JSON: `losselot db graph`
- `commands` - Show recent command log: `losselot db commands`
- `view` - Open web viewer: `losselot serve . --port 3001` then navigate to /graph

### Create Nodes
- `goal <title>` - Create a goal node: `losselot db add-node -t goal "<title>"`
- `decision <title>` - Create a decision point: `losselot db add-node -t decision "<title>"`
- `option <title>` - Create an option node: `losselot db add-node -t option "<title>"`
- `action <title>` - Create an action node: `losselot db add-node -t action "<title>"`
- `outcome <title>` - Create an outcome node: `losselot db add-node -t outcome "<title>"`
- `observation <title>` - Create an observation: `losselot db add-node -t observation "<title>"`

### Create Edges
- `link <from> <to>` - Link two nodes: `losselot db add-edge <from> <to>`
- `link <from> <to> -t <type>` - Link with type (leads_to, requires, chosen, rejected, blocks, enables)

### Update Status
- `status <id> <status>` - Update node status (pending, active, completed, rejected)

### Backup
- `backup` - Create timestamped backup: `losselot db backup`

## Node Types Explained
- **goal**: High-level objective (e.g., "Improve lo-fi detection accuracy")
- **decision**: A choice point with multiple options
- **option**: A possible approach to consider
- **action**: Something we implemented or will implement
- **outcome**: Result of an action (success/failure/learning)
- **observation**: Data point or finding from testing

## Edge Types Explained
- **leads_to**: Natural progression (goal → decision)
- **requires**: Dependency relationship
- **chosen**: Selected option from a decision
- **rejected**: Option that was not selected (with rationale)
- **blocks**: Something preventing progress
- **enables**: Something that makes another thing possible

## Example Workflow
```bash
# Start a new investigation
losselot db add-node -t goal "Detect cassette tape lo-fi properly"
losselot db add-node -t decision "Choose cutoff detection method"
losselot db add-node -t option "Spectral slope analysis"
losselot db add-node -t option "Cutoff variance measurement"

# Link them
losselot db add-edge 1 2 -t leads_to
losselot db add-edge 2 3 -t leads_to
losselot db add-edge 2 4 -t leads_to

# Record a choice
losselot db add-edge 2 3 -t chosen -r "Better handles gradual rolloff"
losselot db add-edge 2 4 -t rejected -r "Too sensitive to natural variation"
```

Execute the appropriate command based on the user's request in $ARGUMENTS.
