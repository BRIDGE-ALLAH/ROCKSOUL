import { useCallback, useEffect, useMemo, useState, type FormEvent } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

type Project = {
  id: string;
  name: string;
  rootPath: string;
  createdAt: string;
  updatedAt: string;
};

type Mission = {
  id: string;
  projectId: string;
  title: string;
  objective: string;
  status: string;
  createdAt: string;
  updatedAt: string;
};

type Artifact = {
  id: string;
  missionId: string;
  kind: string;
  title: string;
  content: string;
  sha256: string;
  createdAt: string;
};

type Evidence = {
  id: string;
  missionId: string;
  artifactId: string | null;
  kind: string;
  summary: string;
  sourcePath: string | null;
  detailsJson: string;
  createdAt: string;
};

type InspectionResult = {
  artifact: Artifact;
  evidence: Evidence;
  fileCount: number;
  truncated: boolean;
};

function formatDate(value: string): string {
  return new Date(value).toLocaleString();
}

function desktopOnly(): void {
  if (!isTauri()) {
    throw new Error("Open Rocksoul as the native desktop app to use local project tools.");
  }
}

export default function App() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [missions, setMissions] = useState<Mission[]>([]);
  const [artifacts, setArtifacts] = useState<Artifact[]>([]);
  const [evidence, setEvidence] = useState<Evidence[]>([]);
  const [projectId, setProjectId] = useState("");
  const [missionId, setMissionId] = useState("");
  const [projectName, setProjectName] = useState("");
  const [rootPath, setRootPath] = useState("");
  const [missionTitle, setMissionTitle] = useState("Map this workspace");
  const [objective, setObjective] = useState(
    "Create a safe, read-only inventory of this project's structure.",
  );
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [vaultState, setVaultState] = useState<"preview" | "connecting" | "ready" | "error">(
    isTauri() ? "connecting" : "preview",
  );
  const [notice, setNotice] = useState(() =>
    isTauri()
      ? "Connecting to the local vault..."
      : "Browser preview only. Open the desktop app to connect local tools.",
  );

  const inDesktop = isTauri();
  const selectedProject = useMemo(
    () => projects.find((project) => project.id === projectId) ?? null,
    [projects, projectId],
  );
  const selectedMission = useMemo(
    () => missions.find((mission) => mission.id === missionId) ?? null,
    [missions, missionId],
  );
  const latestArtifact = artifacts[0] ?? null;

  const refreshProjects = useCallback(async (preferredProjectId?: string) => {
    desktopOnly();
    const nextProjects = await invoke<Project[]>("list_projects");
    setProjects(nextProjects);

    const nextProjectId =
      preferredProjectId && nextProjects.some((project) => project.id === preferredProjectId)
        ? preferredProjectId
        : nextProjects[0]?.id ?? "";

    setProjectId(nextProjectId);
    if (!nextProjectId) {
      setMissions([]);
      setMissionId("");
      setArtifacts([]);
      setEvidence([]);
      return;
    }

    const nextMissions = await invoke<Mission[]>("list_missions", {
      projectId: nextProjectId,
    });
    setMissions(nextMissions);
    const nextMissionId = nextMissions[0]?.id ?? "";
    setMissionId(nextMissionId);

    if (!nextMissionId) {
      setArtifacts([]);
      setEvidence([]);
      return;
    }

    const [nextArtifacts, nextEvidence] = await Promise.all([
      invoke<Artifact[]>("list_artifacts", { missionId: nextMissionId }),
      invoke<Evidence[]>("list_evidence", { missionId: nextMissionId }),
    ]);
    setArtifacts(nextArtifacts);
    setEvidence(nextEvidence);
  }, []);

  useEffect(() => {
    if (!inDesktop) {
      setVaultState("preview");
      return;
    }

    setVaultState("connecting");
    void refreshProjects()
      .then(() => {
        setVaultState("ready");
        setNotice("Local vault connected. Projects are stored on this device.");
      })
      .catch((cause: unknown) => {
        setVaultState("error");
        setError(String(cause));
        setNotice("The local vault could not be opened.");
      });
  }, [inDesktop, refreshProjects]);

  async function handleSelectProject(nextProjectId: string) {
    setProjectId(nextProjectId);
    setMissionId("");
    setArtifacts([]);
    setEvidence([]);
    setError("");

    if (!nextProjectId) {
      setMissions([]);
      return;
    }

    try {
      const nextMissions = await invoke<Mission[]>("list_missions", {
        projectId: nextProjectId,
      });
      setMissions(nextMissions);
      const nextMissionId = nextMissions[0]?.id ?? "";
      setMissionId(nextMissionId);
      if (nextMissionId) {
        const [nextArtifacts, nextEvidence] = await Promise.all([
          invoke<Artifact[]>("list_artifacts", { missionId: nextMissionId }),
          invoke<Evidence[]>("list_evidence", { missionId: nextMissionId }),
        ]);
        setArtifacts(nextArtifacts);
        setEvidence(nextEvidence);
      }
    } catch (cause) {
      setError(String(cause));
    }
  }

  async function handleSelectMission(nextMissionId: string) {
    setMissionId(nextMissionId);
    setError("");
    if (!nextMissionId) {
      setArtifacts([]);
      setEvidence([]);
      return;
    }

    try {
      const [nextArtifacts, nextEvidence] = await Promise.all([
        invoke<Artifact[]>("list_artifacts", { missionId: nextMissionId }),
        invoke<Evidence[]>("list_evidence", { missionId: nextMissionId }),
      ]);
      setArtifacts(nextArtifacts);
      setEvidence(nextEvidence);
    } catch (cause) {
      setError(String(cause));
    }
  }

  async function handleBrowseFolder() {
    if (!inDesktop || busy) return;
    setError("");
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Choose a local project folder",
      });
      if (typeof selected === "string") {
        setRootPath(selected);
        setNotice("Folder selected. Link it to create a local project.");
      }
    } catch (cause) {
      setError(String(cause));
    }
  }

  async function handleCreateProject(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setBusy(true);
    setError("");
    try {
      desktopOnly();
      const project = await invoke<Project>("create_project", {
        name: projectName,
        rootPath,
      });
      setProjectName("");
      setRootPath("");
      await refreshProjects(project.id);
      setNotice(`Project linked: ${project.name}. No workspace files were changed.`);
    } catch (cause) {
      setError(String(cause));
    } finally {
      setBusy(false);
    }
  }

  async function handleCreateMission(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!selectedProject) return;
    setBusy(true);
    setError("");
    try {
      desktopOnly();
      const mission = await invoke<Mission>("create_mission", {
        projectId: selectedProject.id,
        title: missionTitle,
        objective,
      });
      const nextMissions = await invoke<Mission[]>("list_missions", {
        projectId: selectedProject.id,
      });
      setMissions(nextMissions);
      setMissionId(mission.id);
      const [nextArtifacts, nextEvidence] = await Promise.all([
        invoke<Artifact[]>("list_artifacts", { missionId: mission.id }),
        invoke<Evidence[]>("list_evidence", { missionId: mission.id }),
      ]);
      setArtifacts(nextArtifacts);
      setEvidence(nextEvidence);
      setMissionTitle("");
      setNotice("Mission drafted. No tools have run.");
    } catch (cause) {
      setError(String(cause));
    } finally {
      setBusy(false);
    }
  }

  async function handleInspect() {
    if (!selectedProject || !selectedMission) return;
    setBusy(true);
    setError("");
    try {
      desktopOnly();
      const result = await invoke<InspectionResult>("inspect_workspace", {
        projectId: selectedProject.id,
        missionId: selectedMission.id,
      });
      const [nextMissions, nextArtifacts, nextEvidence] = await Promise.all([
        invoke<Mission[]>("list_missions", { projectId: selectedProject.id }),
        invoke<Artifact[]>("list_artifacts", { missionId: selectedMission.id }),
        invoke<Evidence[]>("list_evidence", { missionId: selectedMission.id }),
      ]);
      setMissions(nextMissions);
      setArtifacts(nextArtifacts);
      setEvidence(nextEvidence);
      setNotice(
        `Sound check ready: ${result.fileCount} file entries inventoried${result.truncated ? " (limit reached)" : ""}. Workspace remains untouched.`,
      );
    } catch (cause) {
      setError(String(cause));
    } finally {
      setBusy(false);
    }
  }

  async function handleApproveMission() {
    if (!selectedMission || selectedMission.status !== "review") return;
    setBusy(true);
    setError("");
    try {
      desktopOnly();
      const updated = await invoke<Mission>("approve_mission", {
        missionId: selectedMission.id,
      });
      setMissions((current) => current.map((mission) =>
        mission.id === updated.id ? updated : mission
      ));
      const [nextArtifacts, nextEvidence] = await Promise.all([
        invoke<Artifact[]>("list_artifacts", { missionId: updated.id }),
        invoke<Evidence[]>("list_evidence", { missionId: updated.id }),
      ]);
      setArtifacts(nextArtifacts);
      setEvidence(nextEvidence);
      setNotice("Mission approved. The artifact and its evidence stay on this device.");
    } catch (cause) {
      setError(String(cause));
    } finally {
      setBusy(false);
    }
  }

  return (
    <main className="app-shell">
      <header className="topbar">
        <a className="wordmark" href="#" aria-label="Rocksoul home">
          ROCK<span>SOUL</span><i>®</i>
        </a>
        <div className="topline">CREATIVE INTELLIGENCE / LOCAL FIRST</div>
        <div className={vaultState === "ready" ? "status-chip online" : vaultState === "error" ? "status-chip error-state" : "status-chip"}>
          <span className="status-dot" />
          {vaultState === "ready"
            ? "LOCAL VAULT"
            : vaultState === "connecting"
              ? "CONNECTING"
              : vaultState === "error"
                ? "VAULT ERROR"
                : "PREVIEW MODE"}
        </div>
      </header>

      <section className="hero">
        <div className="hero-copy">
          <p className="eyebrow">FIELD NOTE 001 / FIRST LOCAL SLICE</p>
          <h1>THINK WITH<br /><em>CHAOS.</em><br />ACT WITH RIOT.</h1>
          <p className="hero-text">
            Bring a project. Shape a mission. Inspect the terrain. Make the first artifact.
            No prompt theatre. No silent file changes.
          </p>
        </div>
        <div className="hero-stamp" aria-label="Work hard, play hard">
          <span>WORK HARD</span>
          <strong>✳</strong>
          <span>PLAY HARD</span>
        </div>
      </section>

      {!inDesktop && (
        <div className="preview-banner" role="status">
          Browser preview only. Run <code>bun run desktop:dev</code> to connect the local Rust vault.
        </div>
      )}

      <div className="status-line" role="status">
        <span className="status-icon">↳</span>
        {notice}
      </div>
      {error && <div className="error-banner" role="alert">{error}</div>}

      <section className="workbench">
        <aside className="panel wall">
          <div className="panel-heading">
            <span className="section-index">01 / THE WALL</span>
            <span className="panel-mark">✳</span>
          </div>
          <h2>YOUR<br />PROJECTS<span>.</span></h2>

          <form className="stack-form" onSubmit={handleCreateProject}>
            <label>
              Project name
              <input
                value={projectName}
                onChange={(event) => setProjectName(event.target.value)}
                maxLength={80}
                placeholder="e.g. MoonWitness"
                required
              />
            </label>
            <label>
              Existing folder path
              <div className="path-field">
                <input
                  value={rootPath}
                  onChange={(event) => setRootPath(event.target.value)}
                  placeholder="Choose a local folder..."
                  required
                />
                <button
                  className="browse-button"
                  type="button"
                  onClick={() => void handleBrowseFolder()}
                  disabled={busy || !inDesktop}
                >
                  BROWSE
                </button>
              </div>
            </label>
            <button className="button button-lime" type="submit" disabled={busy || !inDesktop}>
              LINK PROJECT <span>↗</span>
            </button>
          </form>

          <div className="list-label">IN THE ROOM / {projects.length}</div>
          <div className="project-list">
            {projects.map((project) => (
              <button
                className={project.id === projectId ? "project-item active" : "project-item"}
                key={project.id}
                onClick={() => void handleSelectProject(project.id)}
                type="button"
              >
                <span className="project-glyph">▧</span>
                <span className="project-name">
                  {project.name}
                  <small>{project.rootPath}</small>
                </span>
                <span className="project-arrow">↗</span>
              </button>
            ))}
            {projects.length === 0 && (
              <p className="empty-note">No projects yet. Pick a folder and bring it in.</p>
            )}
          </div>
        </aside>

        <section className="panel floor">
          <div className="panel-heading">
            <span className="section-index">02 / THE FLOOR</span>
            <span className="tape-label">MISSION CONTROL</span>
          </div>
          <h2>MAKE A<br />MISSION<span>.</span></h2>
          {selectedProject ? (
            <>
              <div className="selected-project">
                <span>ON THE BENCH</span>
                <strong>{selectedProject.name}</strong>
              </div>
              <form className="stack-form" onSubmit={handleCreateMission}>
                <label>
                  Mission title
                  <input
                    value={missionTitle}
                    onChange={(event) => setMissionTitle(event.target.value)}
                    maxLength={120}
                    placeholder="What are we making?"
                    required
                  />
                </label>
                <label>
                  Objective
                  <textarea
                    value={objective}
                    onChange={(event) => setObjective(event.target.value)}
                    maxLength={2000}
                    rows={3}
                    placeholder="What does a good result look like?"
                    required
                  />
                </label>
                <button
                  className="button button-pink"
                  type="submit"
                  disabled={busy || !inDesktop}
                >
                  DRAFT MISSION <span>↗</span>
                </button>
              </form>
              <div className="list-label">MISSION BOARD / {missions.length}</div>
              <div className="mission-list">
                {missions.map((mission) => (
                  <button
                    className={mission.id === missionId ? "mission-item active" : "mission-item"}
                    key={mission.id}
                    onClick={() => void handleSelectMission(mission.id)}
                    type="button"
                  >
                    <span className="mission-bullet">✳</span>
                    <span>
                      {mission.title}
                      <small>{mission.status.replace("_", " ").toUpperCase()}</small>
                    </span>
                  </button>
                ))}
                {missions.length === 0 && (
                  <p className="empty-note">No mission on the board. Start with one clear objective.</p>
                )}
              </div>
            </>
          ) : (
            <p className="empty-note large">First, link a local project from The Wall.</p>
          )}
        </section>

        <aside className="panel crew">
          <div className="panel-heading">
            <span className="section-index">03 / THE CREW</span>
            <span className="crew-count">01 TOOL</span>
          </div>
          <h2>ONE TOOL.<br />NO NONSENSE<span>.</span></h2>
          <div className="tool-card">
            <div className="tool-icon">⌘</div>
            <div>
              <strong>WORKSPACE SCOUT</strong>
              <small>READ-ONLY / LOCAL</small>
            </div>
            <span className="tool-state">READY</span>
          </div>
          <p className="crew-copy">
            Maps file names, sizes, and types. Respects .gitignore. Skips common secret files,
            build output, dependencies, and symlinks. Does not read file contents.
          </p>
          <div className="permission-list">
            <div><span className="yes">✓</span> List project files</div>
            <div><span className="no">×</span> Read source contents</div>
            <div><span className="no">×</span> Run terminal commands</div>
            <div><span className="no">×</span> Modify project files</div>
            <div><span className="no">×</span> Send data to cloud</div>
          </div>
          <button
            className="button button-ink"
            type="button"
            onClick={() => void handleInspect()}
            disabled={busy || !selectedMission || !inDesktop || selectedMission.status === "review" || selectedMission.status === "completed"}
          >
            {busy ? "WORKING..." : selectedMission?.status === "review" || selectedMission?.status === "completed" ? "SOUND CHECK SAVED" : "RUN SOUND CHECK"} <span>↗</span>
          </button>
          {selectedMission?.status === "review" && (
            <div className="approval-card">
              <strong>THE TAKE IS READY FOR REVIEW.</strong>
              <p>Inspect the inventory and evidence on the right. Approval closes this mission; it does not publish or upload anything.</p>
              <button
                className="button button-lime"
                type="button"
                onClick={() => void handleApproveMission()}
                disabled={busy || !inDesktop}
              >
                {busy ? "SAVING..." : "APPROVE & COMPLETE"} <span>✓</span>
              </button>
            </div>
          )}
          {selectedMission?.status === "completed" && (
            <div className="approval-card approved">
              <strong>MISSION APPROVED ✓</strong>
              <p>Artifact and evidence are stored locally. No project files were changed.</p>
            </div>
          )}
          <p className="tiny-note">Scout writes only to Rocksoul's local database. Approval records your review decision.</p>
        </aside>
      </section>

      <section className="results-grid">
        <div className="panel artifact-panel">
          <div className="panel-heading">
            <span className="section-index">04 / THE FORGE</span>
            <span className="tape-label">ARTIFACT SHELF</span>
          </div>
          <h2>WHAT WE<br />MADE<span>.</span></h2>
          {latestArtifact ? (
            <>
              <div className="artifact-meta">
                <strong>{latestArtifact.title}</strong>
                <span>{formatDate(latestArtifact.createdAt)}</span>
              </div>
              <pre className="artifact-preview">{latestArtifact.content}</pre>
              <div className="hash-line">
                SHA-256 <code>{latestArtifact.sha256}</code>
              </div>
            </>
          ) : (
            <p className="empty-note">No artifact yet. Run the read-only scout to make the first one.</p>
          )}
        </div>

        <div className="panel evidence-panel">
          <div className="panel-heading">
            <span className="section-index">05 / SOUND CHECK</span>
            <span className="evidence-count">{evidence.length} RECEIPTS</span>
          </div>
          <h2>SHOW THE<br />RECEIPTS<span>.</span></h2>
          {evidence.length > 0 ? (
            <div className="evidence-list">
              {evidence.map((item) => (
                <article className="evidence-item" key={item.id}>
                  <div className="evidence-kind">{item.kind.replaceAll("_", " ")}</div>
                  <p>{item.summary}</p>
                  <small>{formatDate(item.createdAt)}</small>
                </article>
              ))}
            </div>
          ) : (
            <p className="empty-note">No evidence yet. A result only counts when it leaves a trace.</p>
          )}
        </div>
      </section>

      <footer className="footer">
        <span>ROCKSOUL / BUILD 001</span>
        <strong>MORE THAN AGI. ONE STEP TOWARD ASI.</strong>
        <span>MADE TO MOVE.</span>
      </footer>
    </main>
  );
}
