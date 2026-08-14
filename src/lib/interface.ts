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

export function joinPath(base: string, name: string) {
    /*
     * This function joins a folder path and a file or folder name
     * into one clean path. It converts Windows backslashes to
     * forward slashes and collapses duplicate slashes, so the
     * result works no matter which OS the user typed the path on.
     *
     * Arguments:
     *    base: string -> the folder path, eg; C:\Games\FNF\ or /home/user/Games
     *    name: string -> the file or folder name to append
     *
     * Returns:
     *    string -> the combined path, eg; C:/Games/FNF/name or /home/user/Games/name
     */
    return (base + "/" + name).replace(/\\/g, "/").replace(/\/+/g, "/");
}
