const messages = {
  shared: {
    brand: {
      name: "IT-Management",
      strapline: "Senior consulting for architecture, delivery, and modernization.",
    },
    navigation: {
      home: "Home",
      services: "Services",
      about: "About",
      cases: "Cases",
      contact: "Contact",
      privacy: "Privacy Policy",
      contactCta: "Start a conversation",
      menu: "Open navigation",
    },
    actions: {
      learnMore: "Learn more",
      viewServices: "View services",
      viewCases: "View cases",
      contact: "Contact IT-Management",
      sendMessage: "Send message",
      readMore: "Read more",
    },
    language: {
      label: "Language",
      options: {
        en: "English",
        da: "Dansk",
      },
    },
    theme: {
      toggle: "Toggle color theme",
      light: "Light theme",
      dark: "Dark theme",
    },
    labels: {
      deliverables: "Deliverables",
      outcomes: "Outcomes",
      context: "Context",
      challenge: "Challenge",
      approach: "Approach",
      outcome: "Outcome",
    },
    footer: {
      description:
        "IT-Management helps leaders stabilize delivery, shape pragmatic architecture, and create a clearer path from strategy to execution.",
      reachOutHeading: "Reach out",
      locationLabel: "Base",
      emailLabel: "Email",
      sitemapHeading: "Navigation",
      rights: "All rights reserved.",
      note: "Placeholder contact details can be replaced in the site configuration.",
    },
  },
  meta: {
    pages: {
      home: {
        title: "Senior IT leadership for architecture, delivery, and change",
        description:
          "IT-Management helps organizations with senior advisory, architecture direction, delivery recovery, and modernization planning.",
      },
      services: {
        title: "Consulting services for leadership, architecture, and delivery",
        description:
          "Explore IT-Management services across technology leadership, architecture, delivery recovery, cloud platforms, and governance.",
      },
      about: {
        title: "About IT-Management",
        description:
          "Learn how IT-Management works with clients through senior hands-on consulting, pragmatic guidance, and long-term delivery perspective.",
      },
      cases: {
        title: "Selected cases and experience",
        description:
          "See example engagements spanning delivery recovery, platform modernization, cloud operating models, and governance improvement.",
      },
      contact: {
        title: "Contact IT-Management",
        description:
          "Get in touch to discuss senior advisory support, architecture guidance, or help stabilizing delivery.",
      },
      privacy: {
        title: "Privacy policy",
        description:
          "Review the current privacy policy structure for IT-Management, including contact, data handling, and retention sections.",
      },
    },
  },
  content: {
    audiences: {
      growth: "Growth companies that need senior guidance without adding permanent overhead",
      midmarket:
        "Mid-market organizations balancing modernization, governance, and delivery pressure",
      regulated:
        "Regulated businesses that need pragmatic structure around risk, compliance, and execution",
      publicSector:
        "Public and critical-function teams that need stakeholder alignment and dependable delivery",
    },
    engagementModels: {
      advisory: "Fractional advisory partnership",
      interim: "Interim leadership support",
      assessment: "Focused assessment and roadmap sprint",
      program: "Program steering and recovery engagement",
    },
    services: {
      "leadership-advisory": {
        title: "Technology leadership advisory",
        summary:
          "Senior sparring for leaders who need clearer priorities, stronger decision support, and steadier execution.",
        description:
          "Ideal when leadership teams need a trusted senior advisor across strategy, governance, prioritization, and stakeholder communication.",
        deliverables: [
          "Leadership decision frameworks",
          "Technology priorities and sequencing",
          "Stakeholder communication material",
        ],
        outcomes: [
          "Clearer direction across business and IT",
          "Better decisions with less noise and rework",
          "More credible communication with executives and teams",
        ],
      },
      "architecture-modernization": {
        title: "Architecture and modernization",
        summary:
          "Pragmatic architecture guidance that helps teams modernize without losing delivery momentum.",
        description:
          "Useful when systems, integrations, or ownership models have become difficult to evolve and need a realistic path forward.",
        deliverables: [
          "Architecture current-state assessment",
          "Target-state principles and roadmap",
          "Modernization decision support",
        ],
        outcomes: [
          "Shared technical direction",
          "Reduced architecture risk in key initiatives",
          "More actionable modernization planning",
        ],
      },
      "delivery-recovery": {
        title: "Delivery recovery and transformation",
        summary:
          "Fast diagnostic work for programs that are slowed by uncertainty, weak operating rhythm, or unclear accountability.",
        description:
          "Best suited for initiatives that need calm senior intervention to re-establish progress, confidence, and governance.",
        deliverables: [
          "Delivery health assessment",
          "Recovery plan with decision points",
          "Execution cadence and governance reset",
        ],
        outcomes: [
          "Improved delivery confidence",
          "Sharper ownership and escalation paths",
          "Visible progress within a focused time frame",
        ],
      },
      "cloud-platform": {
        title: "Cloud and platform operating model",
        summary:
          "Guidance on how platforms, teams, and governance should work together as cloud usage grows.",
        description:
          "Helpful when cloud adoption is increasing faster than roles, standards, or operational practices.",
        deliverables: [
          "Platform operating model outline",
          "Cloud responsibility split",
          "Guardrails for delivery teams",
        ],
        outcomes: [
          "More coherent cloud decisions",
          "Less friction between platform and product teams",
          "A stronger balance between speed and control",
        ],
      },
      "governance-assurance": {
        title: "Governance, risk, and assurance",
        summary:
          "Lightweight but credible structures for organizations that need better oversight without creating drag.",
        description:
          "A fit for environments where delivery must be auditable, secure, and explainable to business stakeholders.",
        deliverables: [
          "Governance baseline and decision forums",
          "Risk and dependency visibility",
          "Policy-to-delivery alignment guidance",
        ],
        outcomes: [
          "Stronger control around key initiatives",
          "Better visibility for leadership",
          "A governance model teams can actually work with",
        ],
      },
    },
    cases: {
      "delivery-recovery": {
        title: "Resetting delivery confidence in a pressured program",
        summary:
          "A large cross-functional initiative had lost momentum, ownership clarity, and stakeholder confidence.",
        context:
          "The client faced mounting uncertainty across delivery plans, dependencies, and executive reporting.",
        challenge:
          "Teams were busy, but progress was difficult to explain and risks surfaced too late for useful intervention.",
        approach:
          "A focused review established a realistic picture of delivery health, clarified key decisions, and introduced a steadier operating rhythm.",
        outcome:
          "Leadership regained a credible view of progress, clearer escalations, and a practical plan to stabilize the program.",
      },
      "platform-modernization": {
        title: "Turning a fragmented platform into a modernization roadmap",
        summary:
          "A business-critical landscape had grown organically and become expensive to change with confidence.",
        context:
          "The client needed a path forward that balanced modernization ambition with business continuity.",
        challenge:
          "There was broad agreement that change was needed, but no shared framing for priorities, sequencing, or acceptable trade-offs.",
        approach:
          "The work mapped the current landscape, identified architectural pressure points, and translated findings into a phased roadmap.",
        outcome:
          "The organization gained a clear modernization narrative, better investment decisions, and a calmer path into execution.",
      },
      "cloud-operating-model": {
        title: "Establishing a cloud operating model after rapid growth",
        summary:
          "Cloud adoption had outpaced the internal model for ownership, governance, and support.",
        context:
          "Teams were moving quickly, but standards, platform boundaries, and expectations were inconsistent.",
        challenge:
          "Without a shared operating model, scaling cloud delivery increased both cost and operational risk.",
        approach:
          "The engagement aligned leadership on key roles, platform responsibilities, and practical guardrails for product teams.",
        outcome:
          "Cloud decisions became more consistent, responsibilities became clearer, and platform work gained stronger focus.",
      },
      "compliance-foundation": {
        title: "Strengthening governance for regulated delivery",
        summary:
          "A regulated organization needed better evidence of control without slowing delivery teams unnecessarily.",
        context:
          "Existing governance expectations were broad, but the link between policy and day-to-day delivery work was weak.",
        challenge:
          "Leaders needed visibility and assurance while teams needed simpler, more useful structures.",
        approach:
          "The work defined lightweight forums, clarified decision rights, and created a more usable governance baseline for delivery.",
        outcome:
          "Oversight improved, reporting became more meaningful, and teams could work within clearer expectations.",
      },
    },
  },
  pages: {
    home: {
      hero: {
        eyebrow: "Senior IT consulting",
        title:
          "Architecture, delivery, and leadership support for organizations that need experienced guidance.",
        description:
          "IT-Management helps companies and public-sector teams create clearer direction, stabilize important initiatives, and make technical change more manageable.",
        primaryCta: "Discuss your current priorities",
        secondaryCta: "Explore services",
        highlights: [
          "Senior-level advisory and hands-on consulting",
          "Pragmatic architecture and modernization guidance",
          "Calm support when delivery needs to regain confidence",
        ],
        panelTitle: "Where support creates the most value",
        panelItems: [
          "When leadership needs sharper decisions and clearer trade-offs",
          "When delivery is under pressure and confidence needs rebuilding",
          "When architecture direction must support both change and continuity",
        ],
      },
      services: {
        eyebrow: "What IT-Management helps with",
        title: "Focused support across leadership, architecture, and execution.",
        description:
          "The initial service set is intentionally structured so it can grow over time without changing the core site architecture.",
      },
      proof: {
        eyebrow: "Why clients bring in senior help",
        title: "A calm partner for high-stakes decisions and difficult delivery situations.",
        description:
          "The value is not only technical competence. It is the ability to create clarity, reduce noise, and help stakeholders move with confidence.",
        stats: [
          {
            value: "15+",
            label: "years of senior change and delivery leadership represented in the profile",
          },
          {
            value: "B2B",
            label: "tone and delivery style designed for executive and operational trust",
          },
          {
            value: "2",
            label: "languages supported from the initial architecture",
          },
        ],
        pillars: [
          {
            title: "Pragmatic architecture",
            text: "Recommendations are designed to be useful for real teams, real constraints, and real sequencing decisions.",
          },
          {
            title: "Decision support",
            text: "Leadership gets clearer trade-offs, sharper framing, and communication that works across both business and IT audiences.",
          },
          {
            title: "Delivery steadiness",
            text: "Interventions focus on visibility, ownership, and a healthier execution rhythm instead of noise and theatrics.",
          },
        ],
      },
      cases: {
        eyebrow: "Selected experience",
        title: "Examples of the kinds of situations this work is built for.",
        description:
          "The cases are structured as maintainable content modules so future additions stay consistent and easy to expand.",
        cta: "See all cases",
      },
      principles: {
        eyebrow: "How the work is done",
        title: "Practical, senior, and intentionally low-drama.",
        description:
          "The approach is designed to support leaders and delivery teams without adding complexity or unnecessary process.",
        items: [
          {
            title: "Meet the situation as it is",
            text: "Start with the actual context, constraints, and maturity level instead of forcing a template.",
          },
          {
            title: "Make priorities explainable",
            text: "Useful strategy and architecture guidance should help leaders explain what matters now and what can wait.",
          },
          {
            title: "Leave behind something reusable",
            text: "The result should be clearer structures, stronger decisions, and a better foundation for the next step.",
          },
        ],
      },
      cta: {
        title: "Need a steadier path through architecture or delivery pressure?",
        description:
          "Use the contact page to start a conversation about your current priorities, constraints, and where experienced support would help most.",
        primaryCta: "Go to contact",
        secondaryCta: "View services",
      },
    },
    services: {
      hero: {
        eyebrow: "Services",
        title: "Consulting support that helps leaders move from complexity to clear action.",
        description:
          "The services are designed as reusable building blocks. They can stand alone or combine into a broader engagement depending on the situation.",
      },
      outcomes: {
        eyebrow: "Typical outcomes",
        title: "What clients should expect from the work.",
        description:
          "The emphasis is on direction, decision support, and execution clarity rather than long deliverable lists for their own sake.",
        items: [
          "Clearer priorities and sequencing",
          "More useful architecture and governance decisions",
          "Better visibility into risks, dependencies, and trade-offs",
          "A stronger basis for confident leadership communication",
        ],
      },
      customers: {
        eyebrow: "Who this is for",
        title: "Different organizations, similar pressure points.",
        description:
          "The same underlying consulting strengths are useful across both private and public environments when delivery, architecture, and leadership alignment matter.",
      },
      models: {
        eyebrow: "Engagement models",
        title: "Support can be shaped to fit the need and the pace of the organization.",
        description:
          "The first version keeps engagement models simple, with room to add more explicit offers later if needed.",
      },
      cta: {
        title: "If you already know the situation, let’s discuss the right starting point.",
        description:
          "A short conversation is usually enough to identify whether an advisory, interim, assessment, or recovery-oriented engagement fits best.",
        primaryCta: "Talk about your needs",
      },
    },
    about: {
      hero: {
        eyebrow: "About",
        title: "A senior consulting profile built around clarity, trust, and useful progress.",
        description:
          "IT-Management is positioned as a credible partner for leaders who need experienced guidance across architecture, delivery, and organizational change.",
      },
      story: {
        eyebrow: "Company introduction",
        title: "Independent senior consulting with a practical working style.",
        paragraphs: [
          "The company is built to support leaders and teams when the work is important, complex, and hard to simplify from the outside.",
          "That means combining strategic perspective with enough operational understanding to help decisions hold up in delivery reality.",
        ],
      },
      profile: {
        eyebrow: "Founder profile",
        title: "Senior advisor and hands-on consulting partner",
        role: "Founder and lead consultant",
        summary:
          "The profile combines leadership advisory, architecture understanding, and delivery experience to support organizations through decision-heavy change.",
        focusAreas: [
          "Bridging executive goals with technical reality",
          "Creating calmer, more useful decision structures",
          "Supporting programs and teams through ambiguity and pressure",
        ],
      },
      principles: {
        eyebrow: "Working principles",
        title: "What clients can expect from the collaboration.",
        description:
          "The aim is to be constructive, clear, and dependable in situations where noise is already high.",
        items: [
          {
            title: "Clarity over jargon",
            text: "Communication should help decisions move forward, not create distance or confusion.",
          },
          {
            title: "Progress over performance theater",
            text: "The work should improve direction and execution, not add unnecessary ceremony.",
          },
          {
            title: "Respect for operational reality",
            text: "Good recommendations account for constraints, stakeholders, and the actual capacity of the organization.",
          },
        ],
      },
      experience: {
        eyebrow: "Experience",
        title: "Seniority grounded in real delivery conditions.",
        description:
          "The site starts with a concise public profile, but the structure is ready for future expansion with deeper credentials, industry specifics, and references.",
        stats: [
          {
            value: "Strategy",
            label: "Technology direction and leadership support",
          },
          {
            value: "Architecture",
            label: "Modernization guidance and structural clarity",
          },
          {
            value: "Execution",
            label: "Delivery recovery, governance, and operating rhythm",
          },
        ],
      },
      cta: {
        title: "If this working style matches what you need, the next step is simple.",
        description:
          "Use the contact page to outline your current challenge, timing, and where a senior external perspective would help.",
        primaryCta: "Contact IT-Management",
      },
    },
    cases: {
      hero: {
        eyebrow: "Cases and experience",
        title: "Structured examples of the challenges this consulting work is designed to support.",
        description:
          "The current cases are representative summaries. They are intentionally structured so they remain easy to edit, reorder, and expand later.",
      },
      overview: {
        eyebrow: "Experience themes",
        title: "Patterns that show up again and again.",
        description:
          "Even when sectors differ, the core issues often revolve around clarity, sequencing, governance, ownership, and technical direction.",
        items: [
          "Programs that need delivery confidence restored",
          "Platform landscapes that need a realistic modernization path",
          "Organizations that need a usable cloud operating model",
          "Regulated environments that need stronger but practical governance",
        ],
      },
      cta: {
        title: "Have a similar challenge in front of you?",
        description:
          "A short conversation can quickly clarify whether the situation calls for advisory support, a diagnostic sprint, or deeper engagement.",
        primaryCta: "Start the conversation",
      },
    },
    contact: {
      hero: {
        eyebrow: "Contact",
        title: "Start with the current challenge, not a polished brief.",
        description:
          "Whether the need is architecture guidance, leadership sparring, or delivery recovery, a concise first message is enough to begin.",
      },
      details: {
        eyebrow: "Contact details",
        title: "Direct and straightforward contact options.",
        description:
          "The current contact details are placeholders that are intentionally easy to replace from the shared configuration.",
        emailLabel: "Email",
        locationLabel: "Base",
        responseLabel: "Typical response",
        responseValue: "Usually within one business day",
        bookingLabel: "Intro call",
        bookingValue: "Booking link placeholder available on request",
      },
      form: {
        title: "Send a message",
        description:
          "Share the situation, timing, and what kind of support you believe would be most useful.",
        fields: {
          name: "Name",
          email: "Email",
          company: "Company (optional)",
          message: "Message",
          honeypot: "Website",
        },
        placeholders: {
          name: "Your name",
          email: "name@company.com",
          company: "Company name",
          message:
            "Briefly describe your context, what is blocked, and what kind of support would help.",
        },
        validation: {
          nameRequired: "Please enter your name.",
          emailRequired: "Please enter your email address.",
          emailInvalid: "Please enter a valid email address.",
          messageRequired: "Please enter a short message.",
        },
        submit: "Send inquiry",
        submitting: "Sending...",
        privacyNote:
          "The current form uses a placeholder submission flow with a clear integration boundary for future backend wiring.",
        successTitle: "Message received",
        successMessage:
          "Thanks for reaching out. The current placeholder flow completed successfully and returned reference {reference}.",
        failureTitle: "Submission placeholder failed",
        failureMessage:
          "The form could not complete the placeholder submission. Please try again or contact by email instead.",
      },
      expectations: {
        title: "What to include in the first message",
        items: [
          "The current challenge or decision you are facing",
          "Whether the pressure is strategic, architectural, or delivery-related",
          "Any relevant timing or stakeholder constraints",
        ],
      },
    },
    privacy: {
      hero: {
        eyebrow: "Privacy policy",
        title: "A replaceable V1 privacy structure with the essential sections in place.",
        description:
          "This initial version uses clear placeholder content so legal review and company-specific wording can be added later without changing the page structure.",
      },
      sections: [
        {
          title: "1. Who we are",
          paragraphs: [
            "IT-Management is the data controller for personal information submitted through this website unless otherwise stated.",
            "If you have questions about privacy or data handling, please use the contact details shown on the contact page.",
          ],
        },
        {
          title: "2. What information we collect",
          paragraphs: [
            "We may collect information you provide directly, including your name, email address, company name, and the content of your inquiry.",
            "We may also collect basic technical information that is typically generated when a website is visited, such as browser, device, and usage data.",
          ],
        },
        {
          title: "3. Why we process personal data",
          paragraphs: [
            "We process information to respond to inquiries, evaluate potential consulting engagements, maintain business communication, and protect the website and its operations.",
            "If additional purposes are introduced later, this policy should be updated before those changes go live.",
          ],
        },
        {
          title: "4. Retention and storage",
          paragraphs: [
            "Personal data should only be kept for as long as there is a legitimate business or legal reason to do so.",
            "Retention periods, storage providers, and any cross-border processing details should be completed when the production handling process is finalized.",
          ],
        },
        {
          title: "5. Your rights",
          paragraphs: [
            "Depending on applicable law, you may have rights related to access, correction, deletion, restriction, objection, and complaint.",
            "Requests should be directed through the contact details published on this website.",
          ],
        },
      ],
    },
  },
} as const;

export default messages;
