import { open } from "@tauri-apps/plugin-dialog";

export type Snackbar = {
    snackbarError: boolean,
    snackbarTime: number,
    givenError: string,
};

export function useSnackbarError(message: string, snackbar: Snackbar) {
    /*
     * This function shows an error snackbar for 5 seconds.
     * It mutates the passed-in snackbar object so the
     * component stays in sync and stays reactive.
     *
     * Arguments:
     *    message: string -> the error message to show
     *    snackbar: object -> the snackbar $state object to mutate
     */
    clearInterval(snackbar.snackbarTime);
    snackbar.snackbarError = true;
    snackbar.givenError = message;

    snackbar.snackbarTime = setInterval(() => {
        snackbar.snackbarError = false;
    }, 5000);
}

export async function pickFile(
    extensions: string[] = ["*"],
    name: string = "File",
    directory: boolean = false,
): Promise<string> {
    /*
     * This function opens a file picker dialog and returns
     * the path of the selected file, or an empty string if cancelled.
     *
     * When extensions is ["*"] (the default), no filter is applied
     * so any file, including .exe and extensionless files, can be picked.
     */
    const file = await open({
        directory,
        multiple: false,
        ...(extensions.length === 1 && extensions[0] === "*" ? {} : {
            filters: [{
                name,
                extensions,
            }],
        }),
    });
    return file ?? "";
}