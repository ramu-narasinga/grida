import type { Metadata } from "next";
import Editor from "../../editor";

export const metadata: Metadata = {
  title: "Grida Canvas",
  description: "Grida Canvas Playground",
};

/**
 * experimental
 * @see https://github.com/gridaco/grida/pull/376
 */
export default async function CanvasPlaygroundPage({
  params,
}: {
  params: Promise<{ room: string }>;
}) {
  const { room } = await params;

  return (
    <main className="w-screen h-screen overflow-hidden">
      <Editor room_id={room} />
    </main>
  );
}
