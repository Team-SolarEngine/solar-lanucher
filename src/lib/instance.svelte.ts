// This constant variable puts all the active terminal into one variable
// so it can be tracked, fetched, removed, and added easily.
export const instanceState = $state({
  instances: [] as { pid: number; workingDir: string, name: string, icon: string, sanitized: string}[]
});

export function addInstanceState(pid: number, workingDir: string, name: string, icon: string, sanitized: string) {
    // This function pushes the args to `instanceState`.
    instanceState.instances.push({ pid, workingDir, name, icon, sanitized });
}

export function removeInstanceState(pid: number) {
    // This function removed the state given by the pid to `instanceState`.
    instanceState.instances = instanceState.instances.filter(
        (instance) => instance.pid !== pid
    );
}

export function getInstancePids(workingDir: string) {
    // This function gets the pid of the working directory
    // from given from the arguments
    return instanceState.instances
        .filter((instance) => instance.workingDir === workingDir)
        .map((instance) => instance.pid);
}

export function instanceIsRunning(workingDir: string) {
    // This function checks if the instance is running from
    // the arguments given and checking if `instanceState` exists.
    return getInstancePids(workingDir).length > 0;
}