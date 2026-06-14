import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/video_control/')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/video_control/"!</div>
}
