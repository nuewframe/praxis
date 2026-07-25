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
2. **System Development**: This workflow focuses on the technical aspects of
   software development, including architecture, design, and implementation. It
   involves translating business requirements into technical specifications and
   ensuring that the software is built to meet those requirements. Key
   activities include:
   - **Architecture Design**: Creating a blueprint for the software's structure
     and components, ensuring that it aligns with business requirements and
     technical standards.
   - **Code Implementation**: Writing the actual code that implements the
     software's functionality, following best practices and coding standards.
   - **Integration & Testing**: Ensuring that the different components of the
     software work together seamlessly and meet quality standards through
     rigorous testing and validation processes.
3. **Software Runtime & Delivery**: This workflow focuses on the deployment,
   operation, and maintenance of the software in a production environment. It
   involves ensuring that the software is delivered to users effectively and
   operates reliably. Key activities include:
   - **Deployment**: The process of releasing the software to a production
     environment, ensuring that it is accessible to users and functions as
     intended.
   - **Monitoring & Maintenance**: Continuously monitoring the software's
     performance and addressing any issues that arise, including bug fixes,
     updates, and enhancements.
   - **User Support**: Providing assistance to users, addressing their concerns,
     and gathering feedback to inform future development efforts.

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
