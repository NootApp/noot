import { TrayIcon } from "@tauri-apps/api/tray";
import { defaultWindowIcon } from "@tauri-apps/api/app";
import { Menu } from "@tauri-apps/api/menu";

let tray: TrayIcon;

export async function configureTrayIcon(): Promise<TrayIcon> {
  console.log("Configuring tray icon");
  console.debug("Existing icon: ", tray !== undefined, tray)
  const menu = await Menu.new({
    items: [
      {
        id: 'quit',
        text: 'Quit'
      }
    ]
  })

  const options = { 
    icon: await defaultWindowIcon(),
    menu,
    menuOnLeftClick: true
  };

  tray = await TrayIcon.new(options);
  return tray;
}
