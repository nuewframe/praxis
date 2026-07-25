## Observed reality of Software Development Workflows

There are several deep workflows that are implicitly practiced but not
articulated into a system. Let's explore the three major workflows that are
commonly observed in software development:

1. **Business Process Requirements**: This workflow focuses on the steps and
   processes that a business follows to achieve its objectives. The key
   artifacts produced in this workflow should follow user-centered design
   principles. The workflow involves understanding the business context,
   identifying the key processes, and documenting the requirements that the
   software needs to fulfill. Key activities include:
   - **Event Storming**: A collaborative workshop where stakeholders and
     developers come together to explore and define the business processes and
     requirements.
   - **Thin Slice**: A small, focused piece of functionality that can be
     developed and tested quickly to validate assumptions and gather feedback
     from stakeholders. For example, a thin slice could be a single feature or a
     specific user flow that represents a critical aspect of the software.
   - **Notional Architecture**: A high-level representation of the **System
     Capabilities** and how they interact with each other. It provides an
     overview of the software's structure and components, providing a blueprint
     for development and ensuring alignment with business requirements. The
     notional architecture can be validated through the thin slice approach,
     allowing stakeholders to see how the software will function in practice and
     provide feedback on its design and usability.
   - **Non-Functional Requirements**: Specifications that define the
     performance, security, usability, and other quality attributes of the
     software, ensuring that it meets the necessary standards and expectations.
     This includes considerations for scalability, maintainability, and
     reliability, which are critical for delivering a positive user experience.
     - **Scalability Planning**: Identifying potential growth areas and
       designing the software to handle increased load and complexity.
     - **Sustainability Assessment**: Evaluating the long-term viability of the
       software, including considerations for maintenance, updates, and support.
     - **Performance Optimization**: Ensuring that the software performs
       efficiently under various conditions and meets user expectations.
2. **Agile Blueprint**: An agile blueprint is the result of incremental maturity
   applied to an artifact over many iterations. That means the make up of the
   blueprint are many micro-artifacts that are created and refined over time.
   The blueprint is a living document that evolves as the software development
   progresses, incorporating feedback from stakeholders and adapting to changing
   requirements. It serves as a guide for the development team, providing a
   clear understanding of the software's structure, functionality, and design
   principles. These micro-artifacts focus is a micro-spec that is an atomic
   unit of user value, aka a **thin slice**. The micro-artifacts can include
   user stories, wireframes, prototypes, and other design elements that
   contribute to the overall blueprint. These micro-artifacts mature a systems'
   User Experience Documents, System Design and Architecture, ADR, and code. In
   turn the code matures the logic and the actual living architecture documents
   and user guides. We call this the plan->triage->build->learn->teach cycle in
   the praxis system.

3. **System Development**: ...

4. **Software Runtime**: This exercise focuses on the runtime platform and the
   operational aspects of the software, including deployment, monitoring, and
   maintenance. It involves ensuring that the software runs smoothly in
   production environments and meets performance and reliability standards. Key
   activities include:
   - **Platform Development**: Building and maintaining the underlying
     infrastructure and services that support the software's operation, ensuring
     that it is scalable, secure, and reliable.
   - **Deployment**: The process of releasing the software to production
     environments, ensuring that it is properly configured and ready for use.
   - **Monitoring**: Continuously tracking the software's performance,
     availability, and usage to identify potential issues and areas for
     improvement.
   - **Maintenance**: Ongoing support and updates to the software, addressing
     bugs, security vulnerabilities, and user feedback to ensure that it remains
     functional and relevant over time.

## References

- An **Event Storming** session produces a **Domain Model**, which captures the
  key entities, relationships, and processes within the business domain. This
  model serves as a foundation for understanding the business requirements and
  guiding the development of the software. The thin slice approach allows for
  rapid prototyping and validation of specific features, enabling stakeholders
  to provide feedback early in the development process. User journeys help to
  visualize the user experience and identify areas for improvement, while
  notional architecture provides a high-level overview of the software's
  structure and components. Non-functional requirements ensure that the software
  meets performance, security, and usability standards, contributing to a
  positive user experience.
