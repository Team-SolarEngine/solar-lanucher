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