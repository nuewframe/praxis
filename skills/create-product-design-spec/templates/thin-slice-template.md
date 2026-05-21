# Thin Slice Feature Specification Template

> **Purpose**: Document features as evergreen specifications that remain valid as implementation evolves. Focus on **what** the feature delivers (user value, capabilities, behavior) rather than **how** it's implemented (code, files, tasks). **Key Principle**: Each thin-slice delivers **working functionality** that users can immediately use, ensuring progressive value delivery.

---

## Template Structure

````markdown
# [AREA]-[COMPONENT]-TS-[NUMBER]: [Feature-Name]

> **Status**: Implemented | In Progress | Planned **Version**: 1.0.0

---

## Feature Description

[2-3 sentences describing what this feature provides to users/system. Focus on capabilities and outcomes, not implementation.]

---

## User Value

**As a** [role/persona]\
**I need** [capability]\
**So that** [outcome/benefit]

**Impact**: [Why this matters - business value, user experience, social coordination]

---

## Key Capabilities

### Primary Capabilities

- **[Capability Name]**: [What it enables users to do]
- **[Capability Name]**: [What it enables users to do]
- **[Capability Name]**: [What it enables users to do]

### Behavioral Characteristics

- **[Behavior]**: [How the system behaves in this scenario]
- **[Behavior]**: [How the system behaves in this scenario]

### Quality Attributes

- **[Attribute]**: [Performance, reliability, user experience characteristic]

---

## User Experience

### UI Example

```ts
// Show typical user interaction (replace with your stack)
const result = await <capability>.<action>({
  // ...inputs the user supplies
});
// Expected result: <observable outcome>
```
````

### API Example

```ts
// Show typical API interaction
const response = await fetch("/api/<resource>", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    // ...request payload
  }),
});
// Expected: <status> with <response shape>
```

### Expected Behavior

**Happy Path**:

1. User performs [action]
2. System responds with [behavior]
3. Result: [Observable outcome]

**Error Handling**:

- [Error scenario] → System [recovery behavior]
- [Error scenario] → User sees [helpful message]

---

## Architecture

### Design Pattern

[Name and briefly explain the architectural pattern used - e.g., "Event-driven notifications", "Real-time subscriptions", "Mobile-first responsive design"]

### Component Interaction

```
[Simple text diagram or description of how components interact]

Mobile App → API Call → Database
Notification Service → Push/Fallback → User Device
```

### Key Design Decisions

- **[Decision]**: [Why this approach was chosen]
- **[Decision]**: [What it enables]

### Integration Points

- **[System/Component]**: [How this feature integrates]
- **[System/Component]**: [What interface it uses]

---

## Dependencies & Relationships

### Prerequisites

- [Feature/TS-ID]: [What capability this builds upon]

### Enables

- [Feature/TS-ID]: [What capability this unlocks]

### Related Features

- [Feature/TS-ID]: [How they work together]

---

## Template Instructions

### Purpose of Feature Specifications

Thin-slices are **evergreen feature documentation** that:

- ✅ Describe **what** the feature provides (capabilities, behavior, value)
- ✅ Remain **valid over time** as implementation evolves
- ✅ Help users understand **what** the system can do
- ✅ Guide architects on **why** design decisions were made
- ❌ Do NOT track implementation details (files, line counts, code)
- ❌ Do NOT include TODOs or completion tracking
- ❌ Do NOT list test counts or technical debt

### What to Include

**Feature Description** - The "what":

- Clear description of capabilities provided
- Observable behavior from user perspective
- System characteristics and guarantees

**User Value** - The "why":

- Who benefits and how
- What outcomes this enables
- Business or social impact

**Experience** - The "how it feels":

- Mobile/web interaction examples
- API call examples
- Error messages and recovery

**Architecture** - The "how it works":

- Design patterns used (that remain stable)
- Component interactions
- Key design decisions and rationale
- Integration points with other systems

### What to Exclude

- ❌ Implementation files and line counts
- ❌ Code snippets and implementation details
- ❌ Test counts and test implementation
- ❌ TODOs and work tracking
- ❌ Time estimates and actuals
- ❌ Completion checklists
- ❌ Progress tracking

**Rationale**: Implementation details belong in the code itself (via comments, docs, and tests). Feature specs should remain valid even as implementation changes.

### Naming Convention

```
[AREA]-[COMPONENT]-TS-[NUMBER]: [Feature-Name]

Examples:
- AUTH-USER-TS-001: User Authentication
- EVENT-CREATION-TS-001: Quick Event Creation
- RESPONSE-SYSTEM-TS-001: One-Tap Response
- NOTIFICATION-PUSH-TS-001: Push Notifications
```

### When to Create/Update

**Create New Spec**:

- When designing a new feature
- Before starting implementation
- As documentation for existing features

**Update Existing Spec**:

- When capabilities expand
- When user experience changes
- When architectural patterns evolve
- When design decisions change

**Do NOT Update** for:

- Implementation file changes
- Bug fixes (unless they change behavior)
- Refactoring (unless it changes architecture)
- Performance optimizations (unless they're a key capability)

## Transformation Guide

### For Existing Thin-Slices

When updating existing thin-slices to this format:

1. **Extract Feature Core**:
   - Start with user story and value
   - List key capabilities (what it does)
   - Describe observable behavior

2. **Document Experience**:
   - Keep mobile/web interaction examples
   - Keep API call examples
   - Keep error scenarios

3. **Capture Architecture**:
   - Name the pattern used (e.g., real-time subscriptions, event-driven notifications)
   - Explain component interaction (mobile app ↔ API ↔ database ↔ notifications)
   - Document design decisions (the "why")

4. **Remove Implementation**:
   - Delete file lists and line counts
   - Delete code snippets
   - Delete test counts
   - Delete TODOs and tracking sections
   - Delete time estimates/actuals

5. **Update Status**:
   - Set status to: Implemented | In Progress | Planned
   - Add version number

### Quick Checklist

Before completing a thin-slice update, verify:

- [ ] Focuses on capabilities, not implementation
- [ ] Mobile/web examples show expected user interactions
- [ ] API examples show expected request/response patterns
- [ ] Architecture section explains patterns and decisions
- [ ] No file paths or line counts
- [ ] No code snippets or implementation details
- [ ] No TODOs or completion tracking
- [ ] Will remain valid as code evolves

```
```
