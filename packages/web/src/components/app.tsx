import { useEffect, useRef } from "react";
import { AppSidebar } from "./app-sidebar";
import { SidebarProvider, SidebarTrigger } from "./ui/sidebar";
import {
	CompareSlider,
	CompareSliderAfter,
	CompareSliderBefore,
	CompareSliderHandle,
} from "@/components/ui/compare-slider";
import * as wasm from "../../../effectengine/pkg/effectengine";
import { fileTypeFromBuffer } from 'file-type';

const bytesToBase64 = (bytes: Uint8Array) => {
	// Safer for large arrays
	let binary = '';
	const len = bytes.byteLength;
	for (let i = 0; i < len; i++) {
		binary += String.fromCharCode(bytes[i]);
	}

	const base64String = window.btoa(binary);
	return `data:image/png;base64,${base64String}`;
}

/**
 * Maps file-type extensions to the numeric IDs used in the Rust enum.
 */
const EXTENSION_TO_ID = {
	'avif': 0,
	'bmp': 1,
	'dds': 2,
	'farbfeld': 3,
	'gif': 4,
	'hdr': 5,
	'ico': 6,
	'jpg': 7,
	'jpeg': 7,
	'exr': 8,
	'png': 9,
	'pnm': 10,
	'qoi': 11,
	'tga': 12,
	'tif': 13,
	'tiff': 13,
	'webp': 14
};

async function getImageFormatId(uint8Array: Uint8Array) {
	const type = await fileTypeFromBuffer(uint8Array);

	if (!type) {
  	throw new Error("Could not determine file type from bytes");
	}

	const id = EXTENSION_TO_ID[type.ext as keyof typeof EXTENSION_TO_ID];

	if (id === undefined) {
  	throw new Error(`Format ${type.ext} is not supported by the current Rust matcher`);
	}

	return id;
}

export function App() {
	const newImageRef = useRef<HTMLImageElement>(null);
	const imagePath = "/bird.jpg";

	useEffect(() => {
		if (!newImageRef.current) return;

		// TODO: This blocks the main thread. Outsource to SW?
		const fetchAndProcessImage = async () => {
			const imageData = await (await fetch(imagePath)).bytes();
			const type = await getImageFormatId(imageData);
			const newImage = wasm.bayer2(imageData, type);

			console.log(bytesToBase64(newImage).substring(0, 255));

			newImageRef.current!.src = bytesToBase64(newImage);
		}

		fetchAndProcessImage();
	}, []);

	return (
		<SidebarProvider className="w-fit">
			<main className="w-screen h-screen flex flex-row">
				<AppSidebar />
				<div className="w-full h-full flex flex-col p-4 gap-4">
					<header>
						<SidebarTrigger className="a" />
					</header>
					<div className="w-full h-full flex items-center justify-center">
						<CompareSlider
							defaultValue={50}
							className="h-full overflow-hidden rounded-lg border"
						>
							<CompareSliderBefore>
								<img
									src={imagePath}
									alt="Before"
									className="size-full object-cover"
								/>
							</CompareSliderBefore>
							<CompareSliderAfter>
								<img
									src={imagePath}
									alt="After"
									ref={newImageRef}
									className="size-full object-cover"
								/>
							</CompareSliderAfter>
							<CompareSliderHandle />
						</CompareSlider>
					</div>
				</div>
			</main>
		</SidebarProvider>
	)
}
