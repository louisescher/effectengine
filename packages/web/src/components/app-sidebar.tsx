"use client";

import { GripVertical } from "lucide-react";
import * as React from "react";
import { Button } from "@/components/ui/button";
import {
	Sortable,
	SortableContent,
	SortableItem,
	SortableItemHandle,
	SortableOverlay,
} from "@/components/ui/sortable";
import {
	Sidebar,
	SidebarContent,
	SidebarFooter,
	SidebarGroup,
	SidebarGroupAction,
	SidebarGroupContent,
	SidebarGroupLabel,
	SidebarHeader,
} from "@/components/ui/sidebar"
import { Plus } from "lucide-react";
import { useState } from "react"
import { EffectSettings } from "./effect-settings";

export type ValidEffect = "bayer-2" | "bayer-4" | "bayer-8" | "bayer-16" | "floyd-steinberg" | "kuwahara" | "pixel-sort" | "pixelate" | "quantize" | "white-noise";

export type UsedEffect = { id: number, effect: ValidEffect, title: string };

export function AppSidebar() {
	const [effects, setEffects] = useState<UsedEffect[]>([
		{ id: 1, effect: "bayer-2", title: 'Bayer Dithering (2x2)'},
		{ id: 2, effect: "bayer-4", title: 'Bayer Dithering (4x4)'},
		{ id: 3, effect: "bayer-8", title: 'Bayer Dithering (8x8)'},
		{ id: 4, effect: "bayer-16", title: 'Bayer Dithering (16x16)'},
		{ id: 5, effect: "floyd-steinberg", title: 'Floyd Steinberg Dithering'},
		{ id: 6, effect: "kuwahara", title: 'Kuwahara Filter'},
		{ id: 7, effect: "pixel-sort", title: 'Pixel Sorting'},
		{ id: 8, effect: "pixelate", title: "Pixelation" },
		{ id: 9, effect: "quantize", title: "Color Quantization" },
		{ id: 10, effect: "white-noise", title: "White Noise" },
	]);

	return (
		<Sidebar>
			<SidebarHeader className="px-6 pt-6">
				<img className="w-full h-auto" src="/wordmark.svg" alt="EffectEngine Logo" />
			</SidebarHeader>
			<SidebarContent className="p-4">
				<SidebarGroup>
					<SidebarGroupLabel>Effects</SidebarGroupLabel>
					<SidebarGroupAction title="Add Effect">
						<Plus />
						<span className="sr-only">Add Effect</span>
					</SidebarGroupAction>
					<SidebarGroupContent className="flex flex-col">
						<Sortable
							value={effects}
							onValueChange={setEffects}
							getItemValue={(effect) => effect.id}
						>
							<SortableContent className="flex flex-col gap-1">
								{effects.map((effect) => (
									<SortableItem key={effect.id} value={effect.id} className="flex flex-row items-start gap-2">
										<EffectSettings effect={effect} />
									</SortableItem>
								))}
							</SortableContent>
							<SortableOverlay />
						</Sortable>
					</SidebarGroupContent>
				</SidebarGroup>
			</SidebarContent>
			<SidebarFooter />
		</Sidebar>
	)
}
