import type { TablerIcon } from "@tabler/icons-react";
import {
  IconBriefcase2,
  IconCloudCog,
  IconCompass,
  IconHierarchy2,
  IconLifebuoy,
  IconShieldCheck,
} from "@tabler/icons-react";

export type AudienceId = "growth" | "midmarket" | "regulated" | "publicSector";

export type EngagementModelId =
  | "advisory"
  | "interim"
  | "assessment"
  | "program";

export type ServiceId =
  | "leadership-advisory"
  | "architecture-modernization"
  | "delivery-recovery"
  | "cloud-platform"
  | "governance-assurance";

export interface ServiceDefinition {
  id: ServiceId;
  icon: TablerIcon;
  featured: boolean;
  audienceIds: AudienceId[];
  engagementModelIds: EngagementModelId[];
}

export const serviceDefinitions: ServiceDefinition[] = [
  {
    id: "leadership-advisory",
    icon: IconCompass,
    featured: true,
    audienceIds: ["growth", "midmarket"],
    engagementModelIds: ["advisory", "interim"],
  },
  {
    id: "architecture-modernization",
    icon: IconHierarchy2,
    featured: true,
    audienceIds: ["midmarket", "regulated"],
    engagementModelIds: ["assessment", "program"],
  },
  {
    id: "delivery-recovery",
    icon: IconLifebuoy,
    featured: true,
    audienceIds: ["growth", "midmarket", "publicSector"],
    engagementModelIds: ["assessment", "program"],
  },
  {
    id: "cloud-platform",
    icon: IconCloudCog,
    featured: false,
    audienceIds: ["growth", "midmarket", "regulated"],
    engagementModelIds: ["advisory", "program"],
  },
  {
    id: "governance-assurance",
    icon: IconShieldCheck,
    featured: false,
    audienceIds: ["regulated", "publicSector"],
    engagementModelIds: ["advisory", "interim"],
  },
];

export const featuredServices = serviceDefinitions.filter(
  (service) => service.featured,
);

export const serviceHighlights = [
  {
    id: "leadership",
    icon: IconBriefcase2,
  },
  {
    id: "architecture",
    icon: IconHierarchy2,
  },
  {
    id: "governance",
    icon: IconShieldCheck,
  },
] as const;
