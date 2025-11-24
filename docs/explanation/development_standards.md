# Next Pipeline Standards

## General

In general, we expect teams to follow best practices for whatever language they
are working in. Projects should be in source control with CI/CD.

### General Rules

The following general rules should be followed on all projects:

- **API Endpoints** are versioned and the uri format should be `api/v1/<endpoint>`
- **OpenAPI** documentation should be created for any service with endpoints
- **JSON-RPC** is prefered over **XML-RPC** were applicable
- **Test coverage** should be greater than 80%.
- **Configuration** should be handled by environment variables, and/or command-line options, and/or configuration files.
- **Unit Tests** are required.
- **Documentation** preference is markdown and should follow the Diataxis Framework
- **ULID** prefered over UUID for unique identifiers
- **RFC-3339 Format** prefered for timestamps, use proper format like `2025-11-07T18:12:07.982682Z

## Architecture

Servers should have a versioned API, client library, and a CLI wherever applicable.
Services should be designed to run in container orchestration (like Kubernetes)
and include health checks (readiness, liveness). Any REST APIs should include
OpenAPI documents. Whenever possible, configuration should be handled by
environment variables, and/or command-line options, and/or configuration files.
The Pipeline should be event driven where ever possible.

### Service Checklist

All services and servers should have the following items:

- Unit tests
- Functional tests
- Client library
- CLI
- OpenAPI document (and endpoint for a document)
- README.md

## Functionality

Features and functions for a project should be declared in an ADR. To prevent
scope and feature creep, it is recommended that the project start with the
minimum set of functions and features to accomplish the task. Use the UNIX
philosophy of
["do one thing and do it well"](https://en.wikipedia.org/wiki/Unix_philosophy#Do_One_Thing_and_Do_It_Well).

Accept feature requests and implement after asking "What is the ROI of this
development effort?". Favor simplicity over perfection. Explicit is better than
implicit. Simple is better than complex. Complex is better than complicated.

## Documentation

Projects should use [architectural decision records](https://adr.github.io) to
document architecture design changes made for a project. Projects should contain
at least a _README.md_ file describing the project. Optional files are
_TODO.md_, _CHANGELOG.md_, _INSTALL.md_, and _AUTHORS_ files.

## Repositories

Repositories will have standardized layouts based on the type. The CI files will
be maintained through tooling instead of being curated by hand. The goal is
consistent layouts and automated updates to CI files where applicable.

## Go Projects

Currently we are using Go version [1.25](https://golang.org/doc/go1.25)

[Go Project Layout](https://github.com/golang-standards/project-layout)

We suggest making the use of linters and auto formatting tools a part of the
development process.

Test coverage should be >80%.

## Python Projects

Currently we are supporting Python versions 3.12, 3.13, and 3.14.

Python projects should adhere to the PEP standards.
Python projects should have unit tests and 80% coverage. We suggest running
tests suites with tox or pytest.

## Rust Projects

Rust projects should use the latest stable version of Rust.

## Pre-Commit Hooks and Linters

All projects should use pre-commit hooks and the language appropriate linters in the CI
as part of the evaluation of a pull request.

## Architectural Design Record (ADR)

Projects are encouraged to use ADRs for recording decisions.

- An ADR is a document that captures an important architectural decision made
  along with its context and consequences. For more information, see
  [ADR](https://adr.github.io).
- Keep a collection of records for "architecturally significant" decisions:
  those that affect the structure, non-functional characteristics, dependencies,
  interfaces, or construction techniques.
- Keep ADRs in the project repository under _doc/arch/adr-NNN.md_
- Use a lightweight text formatting language like Markdown or Textile.

## GitHub Pull Requests

Commit messages should follow the conventional commit spec. Commit changes
should reference the Jira ticket in () to enable the integration with Jira
issues.

`git commit -m "fix: some message about fix (JIRA-XXX)"`

## Tagging

We are using [SemVer 2](https://semver.org/spec/v2.0.0.html) for tagging. Use
tagging of the form "v1.2.3", where 1.2.3 is the semantic version. Tag with the
following command:

```bash
git tag -a v1.2.3 -m "Descriptive message 1.2.3"
```

where 1.2.3 is the semantic version.

## Commits

We are using
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)

A specification for adding human- and machine-readable meaning to commit
messages

## Changelogs

We will use
[Conventional Changelog](https://github.com/conventional-changelog/conventional-changelog)
tools to create our release changelogs.

## Copyright

We will follow the [SPDX Spec](https://spdx.github.io/spdx-spec/) for copyright
and licensing information.

### When to Include Copyright Headers

Please include a copyright header on every piece of source code.

For Go

```golang
// SPDX-FileCopyrightText:  2025, SAS Institute Inc., Cary, NC, USA.  All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
```

For Python

```python
# SPDX-FileCopyrightText: 2025, SAS Institute Inc., Cary, NC, USA.  All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
```

For Rust

```rust
// SPDX-FileCopyrightText: 2025, SAS Institute Inc., Cary, NC, USA.  All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
```

### When to Update Copyright Headers

Update the copyright headers in source code based on the following criteria.

If the code **has not changed**, the copyright year should not change.

If the code **changes** keep the original year and add the most recent year,
such as 2019-2020

```golang
// SPDX-FileCopyrightText: 2019-2025, SAS Institute Inc., Cary, NC, USA.  All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
```

## Repositories

### Infrastructure as Code (IaC)

All infrastructure used to Automate Provisioning, Configuration, and Testing
using Continuous Integration/Continuous Delivery should be managed from
distributed version control system (git). Infrastructure as Code is not
Infrastructure as Configuration. IaC should be dynamic and generative. The
ability to produce Infrastructure programmatically is the key.

### gitOps

Also known as "Git as the Source of Truth." By using Git as our main code
source, we can operate almost everything. For example, version control, history,
peer review, and rollback happen through Git without needing to waste time with
other tools. Git becomes the main source for application code, infrastructure,
and configurations.

Everything in this environment is treated as code—source code, configurations
for infrastructure, Docker recipes, Kubernetes configurations, Jenkins
configurations, documentation, and so on. YAML and Markdown are used everywhere.

CI/CD automation is part of gitOps at SAS—automated builds, testing, and
deployment. "Infrastructure-as-code" (IaC) is deployed to run the application
code according to the CaC "configuration-as-code."

## Research

Research issues in Jira should result in documentation of the process, the
pitfalls, the results, and a decision. Research should be time boxed. Research
is finished when you are at a point you then make smart decisions. We will
research, write documentation, and codify the documentation (ie: terraform,
ansible, custom, etc...) where applicable.

## Links

- [12 Factor-App](https://12factor.net)
- [Agile Manifesto](http://agilemanifesto.org)
- [Cloud Native Programming](https://github.com/julz/cloud-native-programming)
- [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
- [Conventional Changelog](https://github.com/conventional-changelog/conventional-changelog)
- [SemVer 2](https://semver.org/spec/v2.0.0.html)
