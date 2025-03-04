"use client";

import React from "react";
import { SearchInput } from "@/components/extension/search-input";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";

export function DataQueryTextSearch({
  onValueChange,
}: {
  onValueChange?: (txt: string) => void;
}) {
  return (
    <Tooltip>
      <TooltipTrigger>
        <SearchInput
          placeholder="Search locally"
          onChange={(e) => onValueChange?.(e.target.value)}
          className="max-w-sm h-7"
          variant="icon"
        />
      </TooltipTrigger>
      <TooltipContent>Local search - Search within loaded data</TooltipContent>
    </Tooltip>
  );
}
