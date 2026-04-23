const messages = {
  shared: {
    brand: {
      name: "IT-Management",
      strapline: "Senior rådgivning inden for arkitektur, leverance og modernisering.",
    },
    navigation: {
      home: "Forside",
      services: "Ydelser",
      about: "Om",
      cases: "Cases",
      contact: "Kontakt",
      privacy: "Privatlivspolitik",
      contactCta: "Tag en dialog",
      menu: "Åbn navigation",
    },
    actions: {
      learnMore: "Læs mere",
      viewServices: "Se ydelser",
      viewCases: "Se cases",
      contact: "Kontakt IT-Management",
      sendMessage: "Send besked",
      readMore: "Se mere",
    },
    language: {
      label: "Sprog",
      options: {
        en: "English",
        da: "Dansk",
      },
    },
    theme: {
      toggle: "Skift farvetema",
      light: "Lyst tema",
      dark: "Mørkt tema",
    },
    labels: {
      deliverables: "Leverancer",
      outcomes: "Resultater",
      context: "Kontekst",
      challenge: "Udfordring",
      approach: "Tilgang",
      outcome: "Resultat",
    },
    footer: {
      description:
        "IT-Management hjælper ledere med at skabe ro omkring leverancer, sætte pragmatisk arkitekturretning og gøre forandringer mere håndterbare.",
      reachOutHeading: "Kontakt",
      locationLabel: "Base",
      emailLabel: "Email",
      sitemapHeading: "Navigation",
      rights: "Alle rettigheder forbeholdes.",
      note:
        "Kontaktoplysningerne er pladsholdere og kan nemt udskiftes i den delte konfiguration.",
    },
  },
  meta: {
    pages: {
      home: {
        title: "Senior IT-ledelse til arkitektur, leverance og forandring",
        description:
          "IT-Management hjælper organisationer med senior rådgivning, arkitekturretning, leveranceredning og moderniseringsplaner.",
      },
      services: {
        title: "Ydelser inden for ledelse, arkitektur og leverance",
        description:
          "Udforsk IT-Managements ydelser inden for teknologiledelse, arkitektur, leveranceredning, cloud-platforme og governance.",
      },
      about: {
        title: "Om IT-Management",
        description:
          "Læs hvordan IT-Management arbejder med kunder gennem senior, hands-on rådgivning, pragmatisk vejledning og stærkt leverancefokus.",
      },
      cases: {
        title: "Udvalgte cases og erfaringer",
        description:
          "Se eksempelopgaver inden for leveranceredning, platform-modernisering, cloud operating model og forbedret governance.",
      },
      contact: {
        title: "Kontakt IT-Management",
        description:
          "Tag kontakt for at drøfte senior rådgivning, arkitekturhjælp eller støtte til at stabilisere leverancer.",
      },
      privacy: {
        title: "Privatlivspolitik",
        description:
          "Gennemgå den aktuelle struktur for IT-Managements privatlivspolitik, herunder kontakt, databehandling og opbevaring.",
      },
    },
  },
  content: {
    audiences: {
      growth:
        "Vækstvirksomheder der har brug for senior sparring uden at øge den faste overhead",
      midmarket:
        "Mellemstore organisationer der balancerer modernisering, governance og leverancepres",
      regulated:
        "Regulerede virksomheder der har brug for pragmatisk struktur omkring risiko, compliance og eksekvering",
      publicSector:
        "Offentlige og samfundskritiske teams der har brug for stærk interessentafstemning og stabile leverancer",
    },
    engagementModels: {
      advisory: "Fraktioneret rådgivningssamarbejde",
      interim: "Interim ledelsesstøtte",
      assessment: "Fokuseret analyse- og roadmap-forløb",
      program: "Programstyring og recovery-forløb",
    },
    services: {
      "leadership-advisory": {
        title: "Rådgivning om teknologiledelse",
        summary:
          "Senior sparring til ledere der har brug for skarpere prioriteringer, bedre beslutningsstøtte og mere stabil eksekvering.",
        description:
          "Velegnet når ledergrupper har brug for en erfaren rådgiver på tværs af strategi, governance, prioritering og interessentkommunikation.",
        deliverables: [
          "Beslutningsrammer for ledelsen",
          "Prioritering og sekventering af teknologiinitiativer",
          "Kommunikationsmateriale til interessenter",
        ],
        outcomes: [
          "Klarere retning mellem forretning og IT",
          "Bedre beslutninger med mindre støj og omarbejde",
          "Mere troværdig kommunikation til ledelse og teams",
        ],
      },
      "architecture-modernization": {
        title: "Arkitektur og modernisering",
        summary:
          "Pragmatisk arkitekturrådgivning der hjælper teams med at modernisere uden at miste leverancemomentum.",
        description:
          "Relevant når systemer, integrationer eller ejerforhold er blevet svære at udvikle videre på og kræver en realistisk vej frem.",
        deliverables: [
          "Kortlægning af nuværende arkitektur",
          "Principper og roadmap for målarkitekturen",
          "Beslutningsstøtte til modernisering",
        ],
        outcomes: [
          "Fælles teknisk retning",
          "Mindre arkitekturrisiko i vigtige initiativer",
          "Mere handlekraftig moderniseringsplanlægning",
        ],
      },
      "delivery-recovery": {
        title: "Leveranceredning og transformation",
        summary:
          "Hurtig diagnostik til programmer der er hæmmet af usikkerhed, svag styringsrytme eller uklart ansvar.",
        description:
          "Passer godt til initiativer der har brug for rolig senior intervention for at genetablere fremdrift, tillid og governance.",
        deliverables: [
          "Analyse af leverancesundhed",
          "Recovery-plan med beslutningspunkter",
          "Reset af governance og eksekveringsrytme",
        ],
        outcomes: [
          "Forbedret tillid til leverancen",
          "Skarpere ansvar og eskaleringsveje",
          "Synlig fremdrift på kortere tid",
        ],
      },
      "cloud-platform": {
        title: "Cloud- og platform operating model",
        summary:
          "Vejledning i hvordan platforme, teams og governance bør fungere sammen i takt med øget cloud-brug.",
        description:
          "Hjælper når cloud-adoption vokser hurtigere end roller, standarder og driftspraksis kan følge med.",
        deliverables: [
          "Skitse til platform operating model",
          "Ansvarsdeling i cloud-landskabet",
          "Praktiske guardrails for leveranceteams",
        ],
        outcomes: [
          "Mere sammenhængende cloud-beslutninger",
          "Mindre friktion mellem platform- og produktteams",
          "Bedre balance mellem hastighed og kontrol",
        ],
      },
      "governance-assurance": {
        title: "Governance, risiko og assurance",
        summary:
          "Letvægtsstrukturer med troværdighed til organisationer der har brug for bedre styring uden at skabe unødig træghed.",
        description:
          "Et godt match for miljøer hvor leverancer skal være auditerbare, sikre og forståelige for forretningsinteressenter.",
        deliverables: [
          "Baseline for governance og beslutningsfora",
          "Synlighed på risiko og afhængigheder",
          "Vejledning mellem politik og leverancepraksis",
        ],
        outcomes: [
          "Stærkere kontrol omkring vigtige initiativer",
          "Bedre ledelsesoverblik",
          "En governance-model teams reelt kan arbejde med",
        ],
      },
    },
    cases: {
      "delivery-recovery": {
        title: "Genopretning af tillid i et presset program",
        summary:
          "Et stort tværgående initiativ havde mistet momentum, ejerskabsklarhed og interessenttillid.",
        context:
          "Kunden oplevede stigende usikkerhed omkring planer, afhængigheder og rapportering til ledelsen.",
        challenge:
          "Teams arbejdede hårdt, men fremdriften var svær at forklare, og risici blev synlige for sent til at kunne håndteres ordentligt.",
        approach:
          "Et fokuseret review skabte et realistisk billede af leverancesituationen, tydeliggjorde nøglebeslutninger og etablerede en roligere styringsrytme.",
        outcome:
          "Ledelsen fik et mere troværdigt billede af fremdriften, klarere eskaleringer og en praktisk plan for at stabilisere programmet.",
      },
      "platform-modernization": {
        title: "Fra fragmenteret platform til moderniseringsroadmap",
        summary:
          "Et forretningskritisk landskab var vokset organisk og blevet dyrt at ændre med sikkerhed.",
        context:
          "Kunden havde brug for en vej frem der balancerede moderniseringsambition med forretningskontinuitet.",
        challenge:
          "Der var bred enighed om behovet for forandring, men ingen fælles ramme for prioritering, sekventering eller acceptable kompromiser.",
        approach:
          "Arbejdet kortlagde landskabet, identificerede de største arkitekturspændinger og omsatte fundene til et trinvist roadmap.",
        outcome:
          "Organisationen fik en klar moderniseringsfortælling, bedre investeringsbeslutninger og en mere rolig vej ind i eksekveringen.",
      },
      "cloud-operating-model": {
        title: "Etablering af cloud operating model efter hurtig vækst",
        summary:
          "Cloud-adoption var løbet foran den interne model for ejerskab, governance og support.",
        context:
          "Teams bevægede sig hurtigt, men standarder, platformgrænser og forventninger var ujævne.",
        challenge:
          "Uden en fælles operating model steg både omkostninger og driftsrisiko i takt med skaleringen.",
        approach:
          "Forløbet samlede ledelsen om roller, platformansvar og praktiske guardrails for produktteams.",
        outcome:
          "Cloud-beslutninger blev mere ensartede, ansvar blev tydeligere, og platformarbejdet fik bedre fokus.",
      },
      "compliance-foundation": {
        title: "Styrket governance i reguleret leverance",
        summary:
          "En reguleret organisation havde brug for bedre dokumentation for kontrol uden at sænke tempoet unødigt.",
        context:
          "Eksisterende governancekrav var brede, men koblingen mellem politik og daglig leverance var svag.",
        challenge:
          "Ledelsen havde brug for overblik og assurance, mens teams havde brug for enklere og mere brugbare strukturer.",
        approach:
          "Arbejdet definerede lette fora, tydeliggjorde beslutningsrettigheder og skabte en mere anvendelig governance-baseline.",
        outcome:
          "Tilsynet blev styrket, rapporteringen blev mere meningsfuld, og teams kunne arbejde inden for tydeligere rammer.",
      },
    },
  },
  pages: {
    home: {
      hero: {
        eyebrow: "Senior IT-rådgivning",
        title:
          "Arkitektur-, leverance- og ledelsesstøtte til organisationer der har brug for erfaren vejledning.",
        description:
          "IT-Management hjælper virksomheder og offentlige teams med at skabe klarere retning, stabilisere vigtige initiativer og gøre tekniske forandringer mere håndterbare.",
        primaryCta: "Drøft jeres aktuelle prioriteter",
        secondaryCta: "Se ydelser",
        highlights: [
          "Senior rådgivning og hands-on konsulentstøtte",
          "Pragmatisk vejledning om arkitektur og modernisering",
          "Rolig støtte når leverancer har brug for at genvinde tillid",
        ],
        panelTitle: "Hvor støtten skaber mest værdi",
        panelItems: [
          "Når ledelsen har brug for skarpere beslutninger og tydeligere prioriteringer",
          "Når leverancer er under pres og tilliden skal genopbygges",
          "Når arkitekturretningen skal understøtte både forandring og kontinuitet",
        ],
      },
      services: {
        eyebrow: "Det IT-Management hjælper med",
        title: "Målrettet støtte på tværs af ledelse, arkitektur og eksekvering.",
        description:
          "Den første servicepakke er bevidst struktureret, så den kan udvides senere uden at ændre den grundlæggende site-arkitektur.",
      },
      proof: {
        eyebrow: "Derfor henter kunder senior støtte ind",
        title: "En rolig partner til vigtige beslutninger og krævende leverancesituationer.",
        description:
          "Værdien ligger ikke kun i teknisk kompetence. Den ligger også i evnen til at skabe klarhed, reducere støj og hjælpe interessenter videre med sikkerhed.",
        stats: [
          {
            value: "15+",
            label: "års repræsenteret senior erfaring med forandring og leveranceledelse",
          },
          {
            value: "B2B",
            label: "tone og udtryk designet til tillid hos ledelse og forretning",
          },
          {
            value: "2",
            label: "sprog understøttet fra den første arkitekturversion",
          },
        ],
        pillars: [
          {
            title: "Pragmatisk arkitektur",
            text: "Anbefalinger skal kunne bruges i virkelige teams med virkelige begrænsninger og beslutninger.",
          },
          {
            title: "Beslutningsstøtte",
            text: "Ledelsen får tydeligere afvejninger, skarpere framing og kommunikation der virker på tværs af forretning og IT.",
          },
          {
            title: "Stabil leverance",
            text: "Indsatserne fokuserer på synlighed, ansvar og en sundere eksekveringsrytme frem for støj og drama.",
          },
        ],
      },
      cases: {
        eyebrow: "Udvalgte erfaringer",
        title: "Eksempler på de situationer denne rådgivning er bygget til.",
        description:
          "Cases er struktureret som vedligeholdbare indholdsmoduler, så fremtidige tilføjelser bliver konsistente og lette at udvide.",
        cta: "Se alle cases",
      },
      principles: {
        eyebrow: "Sådan arbejdes der",
        title: "Praktisk, senior og bevidst uden unødigt drama.",
        description:
          "Tilgangen er designet til at støtte ledere og leveranceteams uden at skabe ekstra kompleksitet eller unødvendig proces.",
        items: [
          {
            title: "Tag udgangspunkt i virkeligheden",
            text: "Start med den faktiske kontekst, begrænsninger og modenhed i stedet for at presse en standardmodel ned over situationen.",
          },
          {
            title: "Gør prioriteringer forklarlige",
            text: "God strategi- og arkitekturrådgivning skal gøre det lettere at forklare hvad der er vigtigt nu, og hvad der kan vente.",
          },
          {
            title: "Efterlad noget der kan bruges videre",
            text: "Resultatet skal være tydeligere strukturer, stærkere beslutninger og et bedre grundlag for næste skridt.",
          },
        ],
      },
      cta: {
        title: "Har I brug for en mere stabil vej gennem arkitektur- eller leverancepres?",
        description:
          "Brug kontaktsiden til at starte en dialog om jeres aktuelle prioriteter, begrænsninger og hvor erfaren støtte vil hjælpe mest.",
        primaryCta: "Gå til kontakt",
        secondaryCta: "Se ydelser",
      },
    },
    services: {
      hero: {
        eyebrow: "Ydelser",
        title: "Konsulentstøtte der hjælper ledere fra kompleksitet til klar handling.",
        description:
          "Ydelserne er designet som genanvendelige byggesten. De kan stå alene eller kombineres i et større forløb afhængigt af situationen.",
      },
      outcomes: {
        eyebrow: "Typiske resultater",
        title: "Hvad kunder typisk skal forvente af samarbejdet.",
        description:
          "Vægten ligger på retning, beslutningsstøtte og eksekveringsklarhed frem for lange leverancelister for deres egen skyld.",
        items: [
          "Klarere prioriteringer og sekventering",
          "Mere brugbare arkitektur- og governancebeslutninger",
          "Bedre synlighed på risici, afhængigheder og kompromiser",
          "Et stærkere grundlag for sikker ledelseskommunikation",
        ],
      },
      customers: {
        eyebrow: "Hvem det er relevant for",
        title: "Forskellige organisationer, men ofte de samme prespunkter.",
        description:
          "De samme konsulentstyrker er nyttige i både private og offentlige miljøer, når leverance, arkitektur og ledelsesalignment er vigtige.",
      },
      models: {
        eyebrow: "Samarbejdsformer",
        title: "Støtten kan tilpasses behovet og organisationens tempo.",
        description:
          "Første version holder samarbejdsmodellerne enkle, med plads til senere at tilføje mere konkrete tilbud hvis det bliver relevant.",
      },
      cta: {
        title: "Hvis situationen allerede er tydelig, kan vi hurtigt finde det rette udgangspunkt.",
        description:
          "En kort samtale er som regel nok til at vurdere om et rådgivningsforløb, interim støtte, analyseforløb eller recovery-indsats passer bedst.",
        primaryCta: "Tal om jeres behov",
      },
    },
    about: {
      hero: {
        eyebrow: "Om",
        title: "En senior konsulentprofil bygget op omkring klarhed, tillid og nyttig fremdrift.",
        description:
          "IT-Management er positioneret som en troværdig partner for ledere der har brug for erfaren støtte på tværs af arkitektur, leverance og organisatorisk forandring.",
      },
      story: {
        eyebrow: "Virksomhedsintroduktion",
        title: "Uafhængig senior rådgivning med en praktisk arbejdsstil.",
        paragraphs: [
          "Virksomheden er bygget til at støtte ledere og teams når arbejdet er vigtigt, komplekst og svært at simplificere udefra.",
          "Det betyder at kombinere strategisk overblik med tilstrækkelig operationel forståelse til at få beslutninger til at holde i leverancevirkeligheden.",
        ],
      },
      profile: {
        eyebrow: "Profil",
        title: "Senior rådgiver og hands-on konsulentpartner",
        role: "Founder og lead consultant",
        summary:
          "Profilen kombinerer ledelsesrådgivning, arkitekturforståelse og leveranceerfaring for at støtte organisationer gennem beslutningstunge forandringer.",
        focusAreas: [
          "Bro mellem ledelsesmål og teknisk virkelighed",
          "Skabelse af roligere og mere brugbare beslutningsstrukturer",
          "Støtte til programmer og teams gennem tvetydighed og pres",
        ],
      },
      principles: {
        eyebrow: "Arbejdsprincipper",
        title: "Det kan kunderne forvente af samarbejdet.",
        description:
          "Målet er at være konstruktiv, tydelig og stabil i situationer hvor støjniveauet allerede er højt.",
        items: [
          {
            title: "Klarhed frem for jargon",
            text: "Kommunikation skal hjælpe beslutninger videre, ikke skabe afstand eller forvirring.",
          },
          {
            title: "Fremdrift frem for performance-teater",
            text: "Arbejdet skal forbedre retning og eksekvering, ikke tilføre unødvendige ceremonier.",
          },
          {
            title: "Respekt for den operationelle virkelighed",
            text: "Gode anbefalinger tager højde for begrænsninger, interessenter og den faktiske kapacitet i organisationen.",
          },
        ],
      },
      experience: {
        eyebrow: "Erfaring",
        title: "Seniority forankret i virkelige leveranceforhold.",
        description:
          "Sitet starter med en koncentreret offentlig profil, men strukturen er klar til senere at blive udvidet med dybere credentials, brancheerfaring og referencer.",
        stats: [
          {
            value: "Strategi",
            label: "Teknologiretning og ledelsesstøtte",
          },
          {
            value: "Arkitektur",
            label: "Moderniseringsvejledning og strukturel klarhed",
          },
          {
            value: "Eksekvering",
            label: "Leveranceredning, governance og driftsrytme",
          },
        ],
      },
      cta: {
        title: "Hvis arbejdsstilen matcher jeres behov, er næste skridt enkelt.",
        description:
          "Brug kontaktsiden til at beskrive udfordringen, tidshorisonten og hvor et erfarent eksternt perspektiv vil hjælpe mest.",
        primaryCta: "Kontakt IT-Management",
      },
    },
    cases: {
      hero: {
        eyebrow: "Cases og erfaring",
        title: "Strukturerede eksempler på de udfordringer denne rådgivning er designet til at støtte.",
        description:
          "De nuværende cases er repræsentative sammendrag. De er bevidst struktureret, så de er lette at redigere, omrokere og udvide senere.",
      },
      overview: {
        eyebrow: "Erfaringstemaer",
        title: "Mønstre der går igen på tværs af opgaver.",
        description:
          "Selv når brancherne er forskellige, handler kerneudfordringerne ofte om klarhed, sekventering, governance, ejerskab og teknisk retning.",
        items: [
          "Programmer der skal genvinde leverancetillid",
          "Platformlandskaber der kræver en realistisk moderniseringsvej",
          "Organisationer der har brug for en anvendelig cloud operating model",
          "Regulerede miljøer der har brug for stærkere, men stadig praktisk governance",
        ],
      },
      cta: {
        title: "Står I med en lignende udfordring?",
        description:
          "En kort samtale kan hurtigt afklare om situationen kalder på rådgivning, en diagnostisk sprint eller et dybere engagement.",
        primaryCta: "Start dialogen",
      },
    },
    contact: {
      hero: {
        eyebrow: "Kontakt",
        title: "Start med den aktuelle udfordring, ikke med et perfekt brief.",
        description:
          "Uanset om behovet er arkitekturrådgivning, ledelsessparring eller leveranceredning, er en kort første besked nok til at komme i gang.",
      },
      details: {
        eyebrow: "Kontaktoplysninger",
        title: "Direkte og enkle kontaktmuligheder.",
        description:
          "De nuværende kontaktoplysninger er pladsholdere, som bevidst er lette at udskifte fra den delte konfiguration.",
        emailLabel: "Email",
        locationLabel: "Base",
        responseLabel: "Typisk svartid",
        responseValue: "Normalt inden for én arbejdsdag",
        bookingLabel: "Intro-samtale",
        bookingValue: "Pladsholder til bookinglink kan aktiveres senere",
      },
      form: {
        title: "Send en besked",
        description:
          "Beskriv situationen, tidshorisonten og hvilken type støtte der vil være mest værdifuld.",
        fields: {
          name: "Navn",
          email: "Email",
          company: "Virksomhed (valgfrit)",
          message: "Besked",
          honeypot: "Website",
        },
        placeholders: {
          name: "Dit navn",
          email: "navn@virksomhed.dk",
          company: "Virksomhedsnavn",
          message:
            "Beskriv kort konteksten, hvad der er blokeret, og hvilken type støtte der vil hjælpe.",
        },
        validation: {
          nameRequired: "Indtast venligst dit navn.",
          emailRequired: "Indtast venligst din emailadresse.",
          emailInvalid: "Indtast venligst en gyldig emailadresse.",
          messageRequired: "Indtast venligst en kort besked.",
        },
        submit: "Send henvendelse",
        submitting: "Sender...",
        privacyNote:
          "Formularen bruger i denne version et pladsholder-flow med en tydelig integrationsgrænse til senere backend-opsætning.",
        successTitle: "Besked modtaget",
        successMessage:
          "Tak for henvendelsen. Pladsholder-flowet blev gennemført og returnerede reference {reference}.",
        failureTitle: "Pladsholderindsendelsen fejlede",
        failureMessage:
          "Formularen kunne ikke gennemføre pladsholderindsendelsen. Prøv igen eller kontakt via email i stedet.",
      },
      expectations: {
        title: "Det er hjælpsomt at skrive",
        items: [
          "Hvilken udfordring eller beslutning I står i lige nu",
          "Om presset primært er strategisk, arkitektonisk eller leverancerelateret",
          "Eventuelle relevante tidsfrister eller interessentkrav",
        ],
      },
    },
    privacy: {
      hero: {
        eyebrow: "Privatlivspolitik",
        title: "En udskiftelig V1-struktur med de vigtigste sektioner på plads.",
        description:
          "Denne første version bruger tydeligt pladsholderindhold, så juridisk gennemgang og virksomhedsspecifik formulering kan tilføjes senere uden at ændre sidestrukturen.",
      },
      sections: [
        {
          title: "1. Hvem vi er",
          paragraphs: [
            "IT-Management er dataansvarlig for personoplysninger indsendt via dette website, medmindre andet er angivet.",
            "Hvis du har spørgsmål om privatliv eller databehandling, kan du bruge kontaktoplysningerne på kontaktsiden.",
          ],
        },
        {
          title: "2. Hvilke oplysninger vi indsamler",
          paragraphs: [
            "Vi kan indsamle oplysninger du selv giver, herunder navn, emailadresse, virksomhedsnavn og indholdet af din henvendelse.",
            "Vi kan også indsamle grundlæggende tekniske oplysninger der typisk dannes ved besøg på et website, såsom browser-, enheds- og brugsdata.",
          ],
        },
        {
          title: "3. Hvorfor vi behandler personoplysninger",
          paragraphs: [
            "Vi behandler oplysninger for at besvare henvendelser, vurdere mulige konsulentopgaver, opretholde forretningskommunikation og beskytte websitet og dets drift.",
            "Hvis der senere indføres yderligere formål, bør denne politik opdateres før ændringerne sættes i produktion.",
          ],
        },
        {
          title: "4. Opbevaring og lagring",
          paragraphs: [
            "Personoplysninger bør kun opbevares så længe der er et legitimt forretningsmæssigt eller juridisk grundlag for det.",
            "Opbevaringsperioder, leverandører og eventuel overførsel til tredjelande bør udfyldes når den endelige produktionsproces er fastlagt.",
          ],
        },
        {
          title: "5. Dine rettigheder",
          paragraphs: [
            "Afhængigt af gældende lovgivning kan du have rettigheder vedrørende indsigt, rettelse, sletning, begrænsning, indsigelse og klage.",
            "Forespørgsler bør sendes via de kontaktoplysninger der er offentliggjort på dette website.",
          ],
        },
      ],
    },
  },
} as const;

export default messages;
