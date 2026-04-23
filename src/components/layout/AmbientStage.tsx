import type { ReactNode } from "react";

import { AmbientBugs } from "@/components/effects/AmbientBugs";
import classes from "@/components/layout/AmbientStage.module.css";

interface AmbientStageProps {
  children: ReactNode;
}

export function AmbientStage({ children }: AmbientStageProps) {
  return (
    <div className={classes.root}>
      <AmbientBugs />
      <div className={classes.content}>{children}</div>
    </div>
  );
}
