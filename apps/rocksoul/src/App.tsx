import { useEffect, useMemo, useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

type Project = {
  id: string;
  name: string;
  workspacePath: string | null;
  createdAt: string;
  updatedAt: string;
};

type Mission = {
  id: string;
  projectId: string;
  title: string;
  brief: string;
  status: "ready" | "active" | "review" | "completed";
  createdAt: string;
  updatedAt: string;
};

type Artifact = {
  id: string;
  missionId: string;
  title: string;
  content: string;
  sha256: string;
  createdAt: string;
};

type Evidence = {
  id: string;
  projectId: string;
  missionId: string | null;
  artifactId: string | null;
  eventType: string;
  summary: string;
  detailsJson: string;
  createdAt: string;
};

type InspectionReport = {
  rootPath: string;
  filesScanned: number;
  directoriesScanned: number;
  skippedSensitive: number;
  skippedSymlinks: number;
  inaccessible: number;
  truncated: boolean;
  extensions: Record<string, number>;
  sampleFiles: string[];
};

function App() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [selectedProjectId, setSelectedProjectId] = useState("");
  const [missions, setMissions] = useState<Mission[]>([]);
  const [selectedMissionId, setSelectedMissionId] = useState("");
  const [artifacts, setArtifacts] = useState<Artifact[]>([]);
  const [evidence, setEvidence] = useState<Evidence[]>([]);
  const [report, setReport] = useState<InspectionReport | null>(null);

  const [projectName, setProjectName] = useState("");
  const [missionTitle, setMissionTitle] = useState("");
  const [missionBrief, setMissionBrief] = useState("");
  const [artifactTitle, setArtifactTitle] = useState("First take");
  const [artifactContent, setArtifactContent] = useState("");

  const [busy, setBusy] = useState("");
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");

  const selectedProject = useMemo(
    () => projects.find((project) => project.id === selectedProjectId) ?? null,
    [projects, selectedProjectId],
  );
  const selectedMission = useMemo(
    () => missions.find((mission) => mission.id === selectedMissionId) ?? null,
    [missions, selectedMissionId],
  );

  async function refreshProjects(preferredId?: string) {
    const rows = await invoke<Project[]>("list_projects");
    setProjects(rows);
    const nextId = preferredId && rows.some((item) => item.id === preferredId)
      ? preferredId
      : selectedProjectId && rows.some((item) => item.id === selectedProjectId)
        ? selectedProjectId
        : rows[0]?.id ?? "";
    setSelectedProjectId(nextId);
  }

  async function refreshProjectData(projectId: string) {
    if (!projectId) {
      setMissions([]);
      setEvidence([]);
      setSelectedMissionId("");
      return;
    }
    const [missionRows, evidenceRows] = await Promise.all([
      invoke<Mission[]>("list_missions", { projectId }),
      invoke<Evidence[]>("list_evidence", { projectId }),
    ]);
    setMissions(missionRows);
    setEvidence(evidenceRows);
    setSelectedMissionId((current) =>
      current && missionRows.some((mission) => mission.id === current)
        ? current
        : missionRows[0]?.id ?? "",
    );
  }

  async function refreshMissionData(missionId: string) {
    if (!missionId) {
      setArtifacts([]);
      return;
    }
    setArtifacts(await invoke<Artifact[]>("list_artifacts", { missionId }));
  }

  useEffect(() => {
    void (async () => {
      try {
        await refreshProjects();
      } catch (cause) {
        setError(errorMessage(cause));
      }
    })();
    // Initial desktop database load only.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (!selectedProjectId) return;
    void refreshProjectData(selectedProjectId).catch((cause) => setError(errorMessage(cause)));
  }, [selectedProjectId]);

  useEffect(() => {
    void refreshMissionData(selectedMissionId).catch((cause) => setError(errorMessage(cause)));
  }, [selectedMissionId]);

  async function runAction(label: string, action: () => Promise<void>) {
    setBusy(label);
    setError("");
    setNotice("");
    try {
      await action();
    } catch (cause) {
      setError(errorMessage(cause));
    } finally {
      setBusy("");
    }
  }

  async function submitProject(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    await runAction("create-project", async () => {
      const project = await invoke<Project>("create_project", { name: projectName });
      setProjectName("");
      await refreshProjects(project.id);
      setNotice("Studio is ready. Drop a spark.");
    });
  }

  async function submitMission(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!selectedProject) return;
    await runAction("create-mission", async () => {
      const mission = await invoke<Mission>("create_mission", {
        projectId: selectedProject.id,
        title: missionTitle,
        brief: missionBrief,
      });
      setMissionTitle("");
      setMissionBrief("");
      await refreshProjectData(selectedProject.id);
      setSelectedMissionId(mission.id);
      setNotice("Mission shaped. Check the brief, then kick it off.");
    });
  }

  async function chooseWorkspace() {
    if (!selectedProject) return;
    await runAction("inspect", async () => {
      const chosen = await open({
        directory: true,
        multiple: false,
        title: "Choose a workspace for read-only inspection",
      });
      if (typeof chosen !== "string") return;
      const result = await invoke<InspectionReport>("inspect_workspace", {
        projectId: selectedProject.id,
        path: chosen,
      });
      setReport(result);
      await refreshProjects(selectedProject.id);
      await refreshProjectData(selectedProject.id);
      setNotice("Inventory saved locally. No file contents were opened or uploaded.");
    });
  }

  async function startMission() {
    if (!selectedMission) return;
    await runAction("start", async () => {
      const updated = await invoke<Mission>("start_mission", { id: selectedMission.id });
      setMissions((rows) => rows.map((item) => item.id === updated.id ? updated : item));
      setNotice("Mission is live. Make the first take.");
    });
  }

  async function submitArtifact(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!selectedMission) return;
    await runAction("save-artifact", async () => {
      await invoke<Artifact>("save_artifact", {
        missionId: selectedMission.id,
        title: artifactTitle,
        content: artifactContent,
      });
      setArtifactContent("");
      await refreshProjectData(selectedMission.projectId);
      await refreshMissionData(selectedMission.id);
      setNotice("Artifact saved. SHA-256 receipt recorded. Ready for review.");
    });
  }

  async function finishMission() {
    if (!selectedMission) return;
    await runAction("complete", async () => {
      const updated = await invoke<Mission>("complete_mission", { id: selectedMission.id });
      setMissions((rows) => rows.map((item) => item.id === updated.id ? updated : item));
      await refreshProjectData(updated.projectId);
      setNotice("Mission completed with an artifact and evidence on record.");
    });
  }

  return (
    <main className="app-shell">
      <header className="topbar">
        <div className="brand-lockup">
          <div className="brand-mark" aria-hidden="true"><span>R</span><i /></div>
          <div>
            <div className="brand-name">ROCKSOUL<span>®</span></div>
            <div className="brand-kicker">AGILITY ENGINE <b>●</b> LOCAL BUILD 0.1</div>
          </div>
        </div>
        <div className="top-mantra">
          <span>THINK WITH CHAOS.</span>
          <strong>ACT WITH RIOT.</strong>
        </div>
        <div className="local-pill"><i /> LOCAL CORE <span>·</span> OFFLINE READY</div>
      </header>

      <div className="workbench">
        <aside className="left-rail">
          <div className="rail-heading"><span>01 / THE WALL</span><span className="rail-symbol">✳</span></div>
          <div className="section-label">YOUR STUDIOS</div>
          <div className="project-list">
            {projects.map((project) => (
              <button
                className={project.id === selectedProjectId ? "project-item selected" : "project-item"}
                key={project.id}
                onClick={() => { setSelectedProjectId(project.id); setReport(null); }}
              >
                <span className="project-glyph">◈</span>
                <span className="project-copy"><b>{project.name}</b><small>{project.workspacePath ? "WORKSPACE LINKED" : "LOCAL STUDIO"}</small></span>
                <span className="project-arrow">↗</span>
              </button>
            ))}
            {projects.length === 0 && <p className="muted-note">No studio yet. Make the first one.</p>}
          </div>

          <form className="create-project" onSubmit={submitProject}>
            <label htmlFor="project-name">START A STUDIO</label>
            <div className="inline-field">
              <input id="project-name" value={projectName} onChange={(event) => setProjectName(event.target.value)} maxLength={80} placeholder="Name your space..." required />
              <button type="submit" aria-label="Create studio" disabled={!!busy}>＋</button>
            </div>
          </form>

          {selectedProject && (
            <div className="workspace-card">
              <div className="section-label">WORKSPACE LINK</div>
              <p>{selectedProject.workspacePath ? shortPath(selectedProject.workspacePath) : "No folder connected yet."}</p>
              <button className="ghost-button" onClick={chooseWorkspace} disabled={!!busy}>
                <span>⌁</span> {busy === "inspect" ? "SCANNING..." : selectedProject.workspacePath ? "RE-SCAN FOLDER" : "CHOOSE FOLDER"}
              </button>
              <small>READ ONLY · NO FILE CONTENTS</small>
            </div>
          )}

          <div className="rail-footer">
            <span className="footer-signal" /> ALL WORK STAYS ON THIS DEVICE
          </div>
        </aside>

        <section className="main-stage">
          <div className="hero-strip">
            <div className="hero-copy">
              <div className="eyebrow"><span>CREATIVE INTELLIGENCE / FIELD NOTES</span><b>NO. 001</b></div>
              <h1>MAKE<br /><em>THINGS</em><br />MOVE<span className="pink-dot">.</span></h1>
              <p>Work hard. Play hard. Make the weird idea real.</p>
            </div>
            <div className="hero-stamp">
              <div className="stamp-ring">MORE<br /><b>THAN</b><br />AGI</div>
              <span>ONE STEP TOWARD ASI →</span>
              <small>BIG HORIZON. SMALL HONEST STEPS.</small>
            </div>
            <div className="hero-halftone" aria-hidden="true" />
          </div>

          {error && <div className="notice error" role="alert"><b>THAT TAKE BROKE.</b><span>{error}</span><button onClick={() => setError("")}>×</button></div>}
          {notice && <div className="notice success" role="status"><b>GOOD MOVE.</b><span>{notice}</span><button onClick={() => setNotice("")}>×</button></div>}

          <div className="stage-heading">
            <div><span className="section-label">02 / THE FLOOR</span><h2>{selectedProject ? selectedProject.name : "YOUR NEXT BIG THING"}</h2></div>
            <div className="stage-index">IDEAS IN MOTION <b>✳</b></div>
          </div>

          {!selectedProject ? (
            <div className="empty-state">
              <div className="empty-icon">✳</div>
              <h3>NO NOISE YET.</h3>
              <p>Spin up a studio. Drop a spark. We'll find the signal.</p>
            </div>
          ) : (
            <div className="stage-grid">
              <div className="mission-column">
                <div className="panel-head"><div><span className="section-label">THE PIT / MISSIONS</span><h3>Pick a fight worth finishing.</h3></div><span className="count-chip">{missions.length.toString().padStart(2, "0")} OPEN</span></div>

                <form className="mission-form" onSubmit={submitMission}>
                  <div className="form-title">NEW MISSION <span>✳</span></div>
                  <input value={missionTitle} onChange={(event) => setMissionTitle(event.target.value)} maxLength={120} placeholder="What are we making real?" required />
                  <textarea value={missionBrief} onChange={(event) => setMissionBrief(event.target.value)} maxLength={12000} placeholder="Give it a little context. What does a good result look like?" rows={3} required />
                  <div className="form-actions"><span>NO AGENT. NO SHELL. JUST YOUR BRIEF.</span><button className="riot-button" type="submit" disabled={!!busy}>DROP THE SPARK ↗</button></div>
                </form>

                <div className="mission-list">
                  {missions.map((mission, index) => (
                    <button key={mission.id} className={selectedMissionId === mission.id ? "mission-item selected" : "mission-item"} onClick={() => setSelectedMissionId(mission.id)}>
                      <span className="mission-number">{String(index + 1).padStart(2, "0")}</span>
                      <span className="mission-info"><b>{mission.title}</b><small>{mission.brief}</small></span>
                      <span className={`status status-${mission.status}`}>{mission.status.toUpperCase()}</span>
                    </button>
                  ))}
                  {missions.length === 0 && <div className="list-empty">No missions in the pit. Start with one clear goal.</div>}
                </div>

                {selectedMission && (
                  <div className="active-mission">
                    <div className="active-top">
                      <div><span className="section-label">SELECTED MISSION</span><h3>{selectedMission.title}</h3></div>
                      <span className={`status status-${selectedMission.status}`}>{selectedMission.status.toUpperCase()}</span>
                    </div>
                    <p className="mission-brief">{selectedMission.brief}</p>

                    {selectedMission.status === "ready" && (
                      <div className="action-row"><span>PLAN IS YOURS. NO TOOLS WILL RUN.</span><button className="dark-button" onClick={startMission} disabled={!!busy}>START MISSION ↗</button></div>
                    )}

                    {selectedMission.status === "active" && (
                      <form className="artifact-form" onSubmit={submitArtifact}>
                        <div className="form-title">FORGE / SAVE AN ARTIFACT <span>↯</span></div>
                        <input value={artifactTitle} onChange={(event) => setArtifactTitle(event.target.value)} maxLength={120} placeholder="Artifact title" required />
                        <textarea value={artifactContent} onChange={(event) => setArtifactContent(event.target.value)} maxLength={1000000} rows={5} placeholder="Write a first result, a research note, or a working plan..." required />
                        <div className="form-actions"><span>LOCAL DATABASE · SHA-256 RECEIPT</span><button className="riot-button" type="submit" disabled={!!busy}>SAVE THE TAKE ↗</button></div>
                      </form>
                    )}

                    {selectedMission.status === "review" && (
                      <div className="review-gate">
                        <div className="review-mark">✓</div>
                        <div><b>THE TAKE IS SAVED.</b><p>Artifact and SHA-256 evidence are on record. Review the result before marking this mission complete.</p></div>
                        <button className="riot-button" onClick={finishMission} disabled={!!busy}>MARK COMPLETE ↗</button>
                      </div>
                    )}
                    {selectedMission.status === "completed" && (
                      <div className="complete-gate"><b>MISSION COMPLETE ✳</b><span>Artifact and evidence are stored locally. Nothing was published or uploaded.</span></div>
                    )}

                    {artifacts.length > 0 && (
                      <div className="artifact-shelf">
                        <div className="section-label">ARTIFACT SHELF / {artifacts.length}</div>
                        {artifacts.map((artifact) => (
                          <div className="artifact-row" key={artifact.id}>
                            <span className="artifact-icon">▤</span>
                            <div><b>{artifact.title}</b><small>SHA-256 · {artifact.sha256.slice(0, 20)}…</small></div>
                            <span>LOCAL</span>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                )}
              </div>

              <aside className="signal-column">
                <div className="signal-card">
                  <div className="panel-head"><div><span className="section-label">SOUND CHECK / 01</span><h3>Workspace signal</h3></div><span className="live-dot" /></div>
                  {report ? (
                    <>
                      <div className="scan-total"><b>{report.filesScanned}</b><span>FILES<br />INVENTORIED</span></div>
                      <div className="scan-stats"><span>{report.directoriesScanned} dirs</span><span>{report.skippedSensitive} sensitive skipped</span><span>{report.skippedSymlinks} links ignored</span></div>
                      {report.truncated && <p className="scan-warning">Scan limit reached. Inventory is partial.</p>}
                      <div className="extensions">
                        {Object.entries(report.extensions).sort((a, b) => b[1] - a[1]).slice(0, 6).map(([ext, count]) => (
                          <div key={ext}><span>{ext}</span><b>{count}</b></div>
                        ))}
                      </div>
                      <div className="sample-label">SAMPLE PATHS · NO CONTENT READ</div>
                      <ul className="sample-files">{report.sampleFiles.slice(0, 8).map((file) => <li key={file}>{file}</li>)}</ul>
                    </>
                  ) : (
                    <div className="scan-empty"><span className="scan-radar">◎</span><p>Connect a workspace to take a read-only inventory. No file contents leave this machine.</p><button className="ghost-button" onClick={chooseWorkspace} disabled={!!busy}>SCAN A WORKSPACE ↗</button></div>
                  )}
                </div>

                <div className="evidence-card">
                  <div className="section-label">RECEIPTS / LOCAL EVIDENCE</div>
                  <div className="evidence-number">{evidence.length.toString().padStart(2, "0")}<span> RECORDS</span></div>
                  {evidence.slice(0, 5).map((item) => (
                    <div className="evidence-row" key={item.id}>
                      <span className="evidence-check">✓</span>
                      <div><b>{item.eventType.replaceAll(".", " ").toUpperCase()}</b><small>{item.summary}</small></div>
                    </div>
                  ))}
                  {evidence.length === 0 && <p className="muted-note">No receipts yet. Inspection and saved artifacts leave a local trail.</p>}
                </div>

                <div className="guardrail-card">
                  <span className="guardrail-icon">⚡</span>
                  <b>CHAOS IN THE IDEAS.<br />DISCIPLINE IN THE ACTIONS.</b>
                  <p>No shell. No agent. No cloud calls in this first slice.</p>
                </div>
              </aside>
            </div>
          )}
        </section>
      </div>
      <footer className="bottom-bar"><span>ROCKSOUL / MAKE INTELLIGENCE MOVE</span><span>WORK HARD <i>✳</i> PLAY HARD</span><span>LOCAL FIRST · HUMAN IN CONTROL</span></footer>
    </main>
  );
}

function errorMessage(cause: unknown): string {
  return cause instanceof Error ? cause.message : String(cause);
}

function shortPath(value: string): string {
  if (value.length <= 34) return value;
  return `…${value.slice(-32)}`;
}

export default App;
