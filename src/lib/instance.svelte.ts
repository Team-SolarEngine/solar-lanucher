export const instanceState = $state({
  instances: [] as { pid: number; workingDir: string, name: string, icon: string, sanitized: string}[]
});

export function addInstanceState(pid: number, workingDir: string, name: string, icon: string, sanitized: string) {
  instanceState.instances.push({ pid, workingDir, name, icon, sanitized });
}

export function removeInstanceState(pid: number) {
  instanceState.instances = instanceState.instances.filter(
    (instance) => instance.pid !== pid
  );
}

export function getInstancePids(workingDir: string) {
  return instanceState.instances
    .filter((instance) => instance.workingDir === workingDir)
    .map((instance) => instance.pid);
}

export function instanceIsRunning(workingDir: string) {
  return getInstancePids(workingDir).length > 0;
}