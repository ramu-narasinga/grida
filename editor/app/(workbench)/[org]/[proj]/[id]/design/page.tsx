"use client";

import React, { useCallback } from "react";
import { AgentThemeProvider } from "@/scaffolds/agent/theme";
import { useEditorState } from "@/scaffolds/editor";
import { SideControl } from "@/scaffolds/sidecontrol";
import FormCollectionPage from "@/theme/templates/formcollection/page";
import { CanvasFloatingToolbar } from "@/scaffolds/canvas-floating-toolbar";
import {
  StandaloneDocumentEditor,
  ViewportRoot,
  EditorSurface,
} from "@/grida-react-canvas";
import { composeEditorDocumentAction } from "@/scaffolds/editor/action";
import { CanvasAction } from "@/grida-react-canvas";

export default function SiteDeisngPage() {
  return (
    <main className="h-full flex flex-1 w-full">
      <CurrentPageCanvas />
    </main>
  );
}

function CurrentPageCanvas() {
  const [state, dispatch] = useEditorState();

  const {
    theme: { lang },
    selected_page_id,
    documents,
  } = state;

  // @ts-ignore
  const document = documents[selected_page_id!];

  const documentDispatch = useCallback(
    (action: CanvasAction) => {
      dispatch(
        composeEditorDocumentAction(
          // @ts-ignore
          selected_page_id!,
          action
        )
      );
    },
    [selected_page_id, dispatch]
  );

  switch (selected_page_id) {
    case "site/dev-collection":
      return <></>;

    default:
      return <>UNKNOWN PAGE {selected_page_id}</>;
  }
}
