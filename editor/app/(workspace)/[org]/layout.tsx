import {
  createServerComponentClient,
  createServerComponentWorkspaceClient,
} from "@/lib/supabase/server";
import { Workspace } from "@/scaffolds/workspace";
import { cookies } from "next/headers";
import { notFound, redirect } from "next/navigation";
import { EditorHelpFab } from "@/scaffolds/help/editor-help-fab";
import WorkspaceSidebar from "@/scaffolds/workspace/sidebar";
import { SidebarProvider } from "@/components/ui/sidebar";

export default async function Layout({
  params,
  children,
}: Readonly<{
  children: React.ReactNode;
  params: { org: string };
}>) {
  const cookieStore = cookies();
  const supabase = createServerComponentClient(cookieStore);
  const wsclient = createServerComponentWorkspaceClient(cookieStore);

  const { data: auth } = await supabase.auth.getUser();

  if (!auth.user) {
    return redirect("/sign-in?next=/" + encodeURIComponent(params.org));
  }

  const { data: organization, error: err } = await wsclient
    .from("organization")
    .select(`*`)
    .eq("name", params.org)
    .single();

  if (err) console.error(err);
  if (!organization) {
    return notFound();
  }

  return (
    <SidebarProvider>
      <Workspace organization={organization}>
        <EditorHelpFab />
        <WorkspaceSidebar />
        {children}
      </Workspace>
    </SidebarProvider>
  );
}
