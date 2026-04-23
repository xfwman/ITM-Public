import type { TablerIcon } from "@tabler/icons-react";
import {
  IconChartArrows,
  IconCloudCog,
  IconDatabaseCog,
  IconShieldCheck,
} from "@tabler/icons-react";

export type CaseId =
  | "delivery-recovery"
  | "platform-modernization"
  | "cloud-operating-model"
  | "compliance-foundation";

export interface CaseDefinition {
  id: CaseId;
  icon: TablerIcon;
  featured: boolean;
}

export const caseDefinitions: CaseDefinition[] = [
  {
    id: "delivery-recovery",
    icon: IconChartArrows,
    featured: true,
  },
  {
    id: "platform-modernization",
    icon: IconDatabaseCog,
    featured: true,
  },
  {
    id: "cloud-operating-model",
    icon: IconCloudCog,
    featured: true,
  },
  {
    id: "compliance-foundation",
    icon: IconShieldCheck,
    featured: false,
  },
];
