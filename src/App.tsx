import { useEffect, useState } from "react";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { getCurrentWindow } from "@tauri-apps/api/window";
import "./App.css";
import {PhysicalPosition} from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { getFormatAtPosition } from "./utils/radial";
import { invoke } from "@tauri-apps/api/core";
import { conversionMap } from "./config/conversions";


function App() {
  const [filePath, setFilePath] = useState<string | null>(null);
  const [fileExtension, setFileExtension] = useState<string | null>(null);
  const [isDragging, setIsDragging] = useState(false);
  const [selectedFormat, setSelectedFormat] = useState<string | null>(null);
  const [gestureDetected, setGestureDetected] = useState(false);
  const [windowGeometry, setWindowGeometry] = useState({
    x: 0,
    y: 0,
    width: 420,
    height: 420,
    scale: 1,
  });
  const [cursorPosition, setCursorPosition] = useState<{ x: number; y: number; } | null>(null);

  const formats = fileExtension ? conversionMap[fileExtension] ?? [] : [];
  const [isConverting, setIsConverting] = useState(false);
  const [conversionMessage, setConversionMessage] = useState("");

  const hoveredFormat = (() => {
  if (!gestureDetected || !cursorPosition || formats.length === 0) {
    return null;
  }
  // Convert screen coordinates to HUD-local CSS coordinates.
  const mouseX = (cursorPosition.x - windowGeometry.x) / windowGeometry.scale;
  const mouseY = (cursorPosition.y - windowGeometry.y) / windowGeometry.scale;
  const centerX = windowGeometry.width / windowGeometry.scale / 2;
  const centerY = windowGeometry.height / windowGeometry.scale / 2;
  const radius = 130;
  const buttonRadius = 32.5;

  for (let index = 0; index < formats.length; index++) {
    const angle = (index * 2 * Math.PI) / formats.length;

    const buttonX = centerX + radius * Math.sin(angle);
    const buttonY = centerY - radius * Math.cos(angle);

    const dx = mouseX - buttonX;
    const dy = mouseY - buttonY;

    if (dx * dx + dy * dy <= buttonRadius * buttonRadius) {
      return formats[index];
    }
  }

  return null;
})();
 
// Gesture detection event listeners.
    useEffect(() => {
      const appWindow = getCurrentWindow();
      let disposed = false;
      let unlistenStart: (() => void) | undefined;
      let unlistenEnd: (() => void) | undefined;
      let unlistenMove: (() => void) | undefined;

      const setup = async () => {
        // Don't intercept clicks on other applications.
        //await appWindow.setIgnoreCursorEvents(true);

        if (disposed) return;

        unlistenStart = await listen<[number, number]>(
          "shift-drag-started",
          async (event) => {
          
            if (disposed) return;

            const [cursorX, cursorY] = event.payload;

            console.log("Gesture started:", cursorX, cursorY);

            setGestureDetected(true);

            // Position the HUD beside the cursor.
            await appWindow.setPosition(
              new PhysicalPosition(
                cursorX + 55,
                cursorY - 210
              )
            );

            if (!disposed) {
              await appWindow.show();
              const position = await appWindow.innerPosition();
              const size = await appWindow.innerSize();
              const scale = await appWindow.scaleFactor();

             setWindowGeometry({
              x: position.x,
              y: position.y,
              width: size.width,
              height: size.height,
              scale,
            });
            }
          }
        );
        
        unlistenEnd = await listen(
          "shift-drag-ended",
          async () => {
            if (disposed) return;

            console.log("Gesture ended");

            setGestureDetected(false);

            await appWindow.hide();
          }
        );
        const unlistenMove = await listen<[number, number]>(
          "shift-drag-moved",
          (event) => {
            const [x, y] = event.payload;

            setCursorPosition({ x, y });
          }
        );

      // If setup was cancelled while listeners were registering.
      if (disposed) {
        unlistenStart?.();
        unlistenEnd?.();
        unlistenMove?.();
      }
    };

    setup().catch(console.error);

    return () => {
      disposed = true;
      unlistenStart?.();
      unlistenEnd?.();
      unlistenMove?.();
    };
  }, []);

  // File drag-and-drop event listener.
    useEffect(() => {
      let unlisten: (() => void) | undefined;
      let cancelled = false;

      async function setupFileDragListener() {
        const cleanup = await getCurrentWebview().onDragDropEvent(
          async(event) => {
            if (event.payload.type === "enter") {
              const paths = event.payload.paths;

              console.log("FILE PATHS:", paths);

              if (paths.length === 0) return;

              const path = paths[0];
              const fileName = path.split(/[\\/]/).pop() ?? "";
              const extension = fileName.split(".").pop()?.toLowerCase() ?? null;

              setFilePath(path);
              setFileExtension(extension);
              setIsDragging(true);
              setSelectedFormat(null);
            }
            if (event.payload.type === "enter") {
              console.log("FILE ENTERED:", event.payload.paths);
            }
            if (event.payload.type === "drop") {
              const { paths, position } = event.payload;

              setIsDragging(false);

              if (paths.length === 0) return;

              const path = paths[0];
              const filename = path.split(/[\\/]/).pop() ?? "";
              const extension = filename.split(".").pop()?.toLowerCase() ?? "";

              console.log("This is the extension:", extension);
              const availableFormats = conversionMap[extension] ?? [];
              console.log("Available formats for this extension:", availableFormats);
              // Tauri's drag position is relative to the webview.
              const centerX = window.innerWidth / 2;
              const centerY = window.innerHeight / 2;

              const chosenFormat = getFormatAtPosition(
                position.x,
                position.y,
                availableFormats,
                centerX,
                centerY
              );

              console.log("SOURCE:", path);
              console.log("OUTPUT FORMAT:", chosenFormat);

              setFilePath(path);
              setFileExtension(extension);
              setSelectedFormat(chosenFormat);


              if (chosenFormat && availableFormats.includes(chosenFormat)) 
              {
                setIsConverting(true);
                setConversionMessage("Converting PNG to JPG...");

                try {
                  // Tauri command that calls the rust function to convert the image
                  const imageExtensions = ["png", "jpg", "jpeg", "webp", "bmp"];
                  const documentExtensions = ["docx", "pdf"];

                  let outputPath: string;

                  if (imageExtensions.includes(extension)) {
                    outputPath = await invoke<string>("convert_image", {
                      inputPath: path,
                      outputFormat: chosenFormat,
                    });
                  } else if (documentExtensions.includes(extension)) {
                    outputPath = await invoke<string>("convert_document", {
                      inputPath: path,
                      outputFormat: chosenFormat,
                    });
                  } else {
                    throw new Error(`Unsupported file type: ${extension}`);
                  }

                  console.log("Converted file:", outputPath);

                  setConversionMessage(`Converted successfully: ${outputPath}`);
                  console.log("CONVERSION COMPLETE:", outputPath);
                } catch (error) {
                  setConversionMessage(`Conversion failed: ${String(error)}`);
                  console.error("CONVERSION ERROR:", error);
                } finally {
                  setIsConverting(false);
                }
              }

              if (chosenFormat) {
                console.log(
                  `Conversion requested: ${extension} → ${chosenFormat}`
                );
              } else {
                console.log("File dropped outside conversion targets.");
              }
            }

            if (event.payload.type === "leave") {
              console.log("FILE LEFT WINDOW");
            }
          }
        );

        if (cancelled) {
          cleanup();
        } else {
          unlisten = cleanup;
        }
      }

      setupFileDragListener().catch(console.error);

      return () => {
        cancelled = true;
        unlisten?.();
      };
    }, []);
    function handleFormatSelect(format: string) {
      if (!filePath) {
        return;
      }

      setSelectedFormat(format);

      console.log("Source file:", filePath);
      console.log("Convert to:", format);
    }
    async function enableClickThrough() {
      await getCurrentWindow().setIgnoreCursorEvents(true);
    }

  return (
    <main className="app">
      <div className="radial-menu">
       <div className="center-circle">
          <span>
            {fileExtension ? fileExtension.toUpperCase() : "FILE"}
          </span>

          <small>
            {isDragging
              ? "Release file"
              : filePath
              ? "File detected"
              : "Drag a file here"}
          </small>
        </div>
        {selectedFormat && fileExtension && (
          <div className="conversion-status">
            {fileExtension.toUpperCase()} → {selectedFormat} selected
          </div>
        
        )}

       {formats.length > 0 ? (
        formats.map((format, index) => {
          const angle = (index * 2 * Math.PI) / formats.length;
          const radius = 130;

          const x = radius * Math.sin(angle);
          const y = -radius * Math.cos(angle);

          return (
            <button
              key={format}
              className={`format-option ${ hoveredFormat === format || selectedFormat === format ? "selected" : ""}`}
              onClick={() => handleFormatSelect(format)}
              style={{
                left: `calc(50% + ${x}px)`,
                top: `calc(50% + ${y}px)`,
                translate: "-50% -50%",
              }}
            >
              {format}
            </button>
            
          );
        })
      ) : (
        <p className="empty-message">
          {filePath
            ? "No conversions available"
            : "Drag a file to see conversion options"}
        </p>
      )}
      </div>
      <div className="gesture-status">
        {gestureDetected
          ? "SHIFT + DRAG DETECTED"
          : "Waiting for Shift + Drag"}
      </div>
      {cursorPosition && (
        <div className="cursor-debug">
          X: {cursorPosition.x} | Y: {cursorPosition.y}
        </div>
      )}
      {conversionMessage && (
        <div className="conversion-result">
          {isConverting ? "Working: " : ""}
          {conversionMessage}
        </div>
      )}
    </main>
  );
}

export default App;