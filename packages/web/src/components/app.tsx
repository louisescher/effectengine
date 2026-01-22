import { AppSidebar } from "./app-sidebar";
import { SidebarProvider, SidebarTrigger } from "./ui/sidebar";

export function App() {
	return (
		<SidebarProvider className="w-fit">
			<main className="w-screen h-screen flex flex-row">
				<AppSidebar />
				<div className="w-full h-full flex flex-col p-4 gap-4">
					<header>
						<SidebarTrigger className="a" />
					</header>
					<div className="w-full h-full flex items-center justify-center">Content</div>
				</div>
			</main>
		</SidebarProvider>
	)
}
