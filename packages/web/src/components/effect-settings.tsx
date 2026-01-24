import { ChevronDown, ChevronLeft, GripVertical } from "lucide-react";
import type { UsedEffect, ValidEffect } from "./app-sidebar";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "./ui/collapsible";
import { SortableItemHandle } from "./ui/sortable";
import { Button } from "./ui/button";
import { useState } from "react";
import {
  Field,
  FieldContent,
  FieldGroup,
  FieldLabel,
} from "@/components/ui/field"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import {
  ColorPicker,
  ColorPickerArea,
  ColorPickerContent,
  ColorPickerEyeDropper,
  ColorPickerFormatSelect,
  ColorPickerHueSlider,
  ColorPickerInput,
  ColorPickerSwatch,
  ColorPickerTrigger,
} from "@/components/ui/color-picker";

const EFFECTS_WITH_SETTINGS: ValidEffect[] = [
	"floyd-steinberg",
	"kuwahara",
	"pixel-sort",
	"pixelate",
	"quantize",
	"white-noise"
];

export function EffectSettings({ effect }: { effect: UsedEffect }) {
	const [open, setOpen] = useState<boolean>(false);

	if (EFFECTS_WITH_SETTINGS.includes(effect.effect)) {
		return (
			<Collapsible className="w-full" open={open} onOpenChange={setOpen}>
				<div className="flex flex-row items-center gap-4 w-full justify-between">
					<div className="flex flex-row items-center gap-4">
						<SortableItemHandle
							className="
								size-8 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted
								aria-expanded:text-foreground focus-visible:border-ring focus-visible:ring-ring/50
								aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive
								dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm
								font-medium focus-visible:ring-[3px] aria-invalid:ring-[3px] [&_svg:not([class*='size-'])]:size-4
								inline-flex items-center justify-center whitespace-nowrap transition-all disabled:pointer-events-none
								disabled:opacity-50 [&_svg]:pointer-events-none shrink-0 [&_svg]:shrink-0 outline-none group/button
								select-none
							"
							data-variant="ghost"
							data-size="icon"
						>
							<GripVertical className="h-4 w-4" />
						</SortableItemHandle>
						<span>{effect.title}</span>
					</div>
					<CollapsibleTrigger asChild>
						<Button variant="ghost" size="icon" className="size-8 ml-auto">
							{open ? (
								<ChevronDown className="h-4 w-4" />
							) : (
								<ChevronLeft className="h-4 w-4" />
							)}
						</Button>
					</CollapsibleTrigger>
				</div>
				<CollapsibleContent className="pl-12">
					<EffectSettingsMapper effect={effect.effect} />
			  </CollapsibleContent>
			</Collapsible>
		)
	}

	return (
		<div className="flex flex-row items-center gap-4 w-full">
			<SortableItemHandle
				className="
					size-8 hover:bg-muted hover:text-foreground dark:hover:bg-muted/50 aria-expanded:bg-muted
					aria-expanded:text-foreground focus-visible:border-ring focus-visible:ring-ring/50
					aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive
					dark:aria-invalid:border-destructive/50 rounded-lg border border-transparent bg-clip-padding text-sm
					font-medium focus-visible:ring-[3px] aria-invalid:ring-[3px] [&_svg:not([class*='size-'])]:size-4
					inline-flex items-center justify-center whitespace-nowrap transition-all disabled:pointer-events-none
					disabled:opacity-50 [&_svg]:pointer-events-none shrink-0 [&_svg]:shrink-0 outline-none group/button
					select-none
				"
				data-variant="ghost"
				data-size="icon"
			>
				<GripVertical className="h-4 w-4" />
			</SortableItemHandle>
			<span>{effect.title}</span>
		</div>
	)
}

function EffectSettingsMapper({ effect }: { effect: ValidEffect }) {
	if (effect === "floyd-steinberg") {
		return <FloydSteinbergOpts />;
	}

	if (effect === "kuwahara") {
		return <KuwaharaOpts />;
	}

	if (effect === "pixel-sort") {
		return <PixelSortOpts />;
	}

	if (effect === "pixelate") {
		return <PixelateOpts />;
	}

	if (effect === "quantize") {
		return <QuantizeOpts />;
	}

	if (effect === "white-noise") {
		return <WhiteNoiseOpts />;
	}

	return <span>Unknown effect.</span>
}

function QuantizeOpts() {
	// TODO: Plus button, add & remove colors, random color each time, preset palettes
	return (
		<ColorPicker>
	    <ColorPickerTrigger className="w-fit h-fit p-0">
	      <ColorPickerSwatch />
	    </ColorPickerTrigger>
	    <ColorPickerContent>
	      <ColorPickerArea />
	      <ColorPickerEyeDropper />
	      <ColorPickerHueSlider />
	      <ColorPickerFormatSelect />
	      <ColorPickerInput withoutAlpha />
	    </ColorPickerContent>
	  </ColorPicker>
	)
}

function FloydSteinbergOpts() {
	// TODO: Dark and light color pickers
	return (
		<div>
			FloydSteinbergOpts
		</div>
	)
}

function PixelSortOpts() {
	return (
 		<FieldGroup className="w-full">
      <Field orientation="vertical" className="gap-1 pt-2">
        <FieldContent>
          <FieldLabel htmlFor="align-item" className="text-sm text-zinc-400">Direction</FieldLabel>
        </FieldContent>
				<Select defaultValue="horizontal">
				  <SelectTrigger className="w-full">
				    <SelectValue placeholder="Mode" />
				  </SelectTrigger>
				  <SelectContent>
				    <SelectItem value="horizontal">Horizontal</SelectItem>
				    <SelectItem value="vertical">Vertical</SelectItem>
				    <SelectItem value="both">Both</SelectItem>
				  </SelectContent>
				</Select>
			</Field>
		</FieldGroup>
	)
}

function KuwaharaOpts() {
	// TODO: Slider for effect size
	return (
		<div>
			KuwaharaOpts
		</div>
	)
}

function PixelateOpts() {
	// TODO: Slider for effect strength
	return (
		<div>
			PixelateOpts
		</div>
	)
}


function WhiteNoiseOpts() {
	// TODO: Slider for effect opacity
	return (
		<div>
			WhiteNoiseOpts
		</div>
	)
}
