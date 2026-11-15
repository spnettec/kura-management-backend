/* Native package delivery through the authenticated host channel. Public IDs
   and steps are saved before writes; credentials and host paths are not saved. */
window.yofcNativePackages = ({ invoke, action, getLanguage, getBusy, onInstalled }) => {
  const $ = id => document.getElementById(id);
  const storageKey = 'yofc.console.native-package.v1';
  const uuid = /^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$/;
  const steps = ['UPLOAD_REQUESTED', 'READY', 'INSTALL_REQUESTED', 'RETIRE_REQUESTED', 'VERIFY_REQUESTED', 'FINALIZE_REQUESTED', 'APPLIED', 'RETIRED'];
  const phases = ['UPLOADING', 'UNOBSERVED', 'ACCEPTED', 'PENDING', 'UNKNOWN', 'APPLIED', 'RETIRED'];
  const labels = {
    zh: {
      nativeTitle: '2. 安装或更新本机中心软件', nativeLaunchTitle: '3. 启动中心设置服务',
      uncheckedCore: '请先读取核定版本及主机状态，确认中心程序后再启动设置服务。',
      absentCore: '中心程序尚未安装。请先上传并安装核定版本，安装后再显式启动设置服务。已有安装数据会保留。',
      verifiedCore: '已安装核心程序，来源摘要核验通过；这不表示服务已启动或初始化已完成。',
      unverifiedCore: '已安装程序或软件包状态尚未核实。暂不启动或读取安装凭据，请保留数据并核对主机安装。',
      nativeHelp: '选择发行清单中的版本并上传对应 deb 文件。已有中心请先在下方维护区域冻结并停服；安装不会自动停止或启动中心。数据库和私有安装数据保留。',
      nativeRefresh: '读取核定版本及主机状态', nativePackageLabel: '核定的中心版本', nativeFileLabel: '中心安装包（deb）',
      nativeUpload: '核对并上传安装包', nativeApproveLabel: '我已核对发行版本和所有本机实例的影响，确认安装后保持服务停止。',
      nativeInstall: '安装核定版本，保持服务停止', nativeObserve: '核对原安装请求', nativeRetire: '取消尚未受理的原请求',
      nativeVerify: '核定原安装成功回执', nativeFinalize: '完成已核定安装的冻结解除', nativeFinish: '已阅，准备下一次安装',
      idle: '尚未发起软件安装。请先读取核定版本和主机状态。', empty: '当前工具包尚未附带适用于本机的核定安装包目录。',
      busyHost: '本机仍有中心进程或未完成的维护，当前不能安装。请在下方维护区域核对全部实例并停服。',
      refreshHost: '启动请求后，原主机停服信息已过期。再次安装前请重新读取核定版本及主机状态。',
      readyHost: '本机中心均已停止，可审阅软件安装。', uploading: '原安装包尚未核实。可核对原请求；不会自动重新上传。',
      ready: '上传文件与核定发行包完全匹配，可确认安装。', pending: '主机正在执行原安装请求，仅核对状态。',
      unknown: '原安装结果尚未核定。保留原编号和安装数据，不重新执行安装。',
      applied: '原安装成功回执和文件校验已通过。安装过程未启动中心；当前运行状态请查看服务状态。',
      held: '安装成功回执已核定，尚需完成保留的冻结解除。不会再次安装或启动中心。',
      retired: '原请求已取消，后到的原安装命令会被拒绝。文件和历史保留。',
      rejected: '本次安装在受理前被明确拒绝。请核对上传文件和主机停服状态。',
      badFile: '所选文件的大小或 SHA-256 与核定版本不一致，未上传或执行安装。',
      pollEnded: '本轮只读核对已结束。可继续核对原请求，不会重复安装。',
      scope: '影响范围：此主机上的所有本机中心实例',
    },
    en: {
      nativeTitle: '2. Install or update native center software', nativeLaunchTitle: '3. Start center setup',
      uncheckedCore: 'Read the approved releases and host state before starting setup.',
      absentCore: 'The center software is not installed. Upload and install an approved release first, then explicitly start setup. Retained installation data is preserved.',
      verifiedCore: 'The installed core program and source digests are verified. This does not mean it is running or initialized.',
      unverifiedCore: 'The installed program or package state is unverified. Keep its data and review the host installation before starting or reading credentials.',
      nativeHelp: 'Select an approved release and upload its deb file. Freeze and stop existing centers in host maintenance below first. Installation never implicitly stops or starts a center. Databases and private installation data are retained.',
      nativeRefresh: 'Read approved releases and host state', nativePackageLabel: 'Approved center release', nativeFileLabel: 'Center package (deb)',
      nativeUpload: 'Verify and upload package', nativeApproveLabel: 'I reviewed the release and the impact on all native instances, and approve installation with services kept stopped.',
      nativeInstall: 'Install approved release; keep services stopped', nativeObserve: 'Observe original installation', nativeRetire: 'Cancel original request before acceptance',
      nativeVerify: 'Verify original successful install receipt', nativeFinalize: 'Finish verified installation fence release', nativeFinish: 'Acknowledge and prepare the next installation',
      idle: 'No software installation was requested. Read the approved releases and host state first.', empty: 'This tools package has no approved release catalog for this architecture.',
      busyHost: 'Native centers or unsettled maintenance are still present. Review and stop every instance in host maintenance before installation.',
      refreshHost: 'The stopped-host snapshot expired after a start request. Read the approved releases and host state again before another installation.',
      readyHost: 'All native centers are stopped. The software installation can be reviewed.', uploading: 'The original upload is unverified. Observe its original request; no upload is automatically repeated.',
      ready: 'The upload exactly matches the approved release. Installation can be confirmed.', pending: 'The host is executing the original installation. Observe only.',
      unknown: 'The original installation outcome is unverified. Retain its ID and data; do not repeat installation.',
      applied: 'The original install receipt and payload are verified. Installation did not start a center. Read service status for its current state.',
      held: 'The successful installation receipt is verified. Finish releasing the retained freeze; this does not install or start a center again.',
      retired: 'The original request was cancelled and rejects delayed installation commands. Files and history are retained.',
      rejected: 'Installation was explicitly refused before acceptance. Review the upload and stopped host state.',
      badFile: 'The file size or SHA-256 differs from the approved release. It was not uploaded or installed.',
      pollEnded: 'This read-only observation ended. Observe the original request again; installation will not be repeated.',
      scope: 'Impact: every natively installed center instance on this host',
    },
  };
  const text = key => labels[getLanguage()][key];
  let saved = null, record = null, overview = null, broken = false, timer = null, generation = 0;
  try {
    const raw = localStorage.getItem(storageKey);
    if (raw) {
      saved = JSON.parse(raw);
      if (!uuid.test(saved.operationId) || !/^[a-z0-9][a-z0-9-]{0,63}$/.test(saved.packageKey) || !steps.includes(saved.step)) throw new Error('invalid');
    }
  } catch { broken = true; }
  const selected = () => overview?.packages?.find(row => row.key === $('nativePackage').value);
  const uncertain = () => !!saved && (!record || ['UNKNOWN', 'UNOBSERVED'].includes(record.phase)
    || record.phase === 'UPLOADING' && ['INSTALL_REQUESTED', 'VERIFY_REQUESTED', 'FINALIZE_REQUESTED', 'RETIRE_REQUESTED'].includes(saved.step));
  function remember(value) {
    localStorage.setItem(storageKey, JSON.stringify(value));
    saved = value;
  }
  function buttons() {
    const busy = getBusy(), pending = record && ['ACCEPTED', 'PENDING'].includes(record.phase);
    const terminal = record && ['APPLIED', 'RETIRED'].includes(record.phase);
    $('nativePackage').disabled = busy || !!saved;
    $('nativeFile').disabled = busy || !!saved;
    $('nativeUpload').disabled = busy || broken || !!saved || !selected() || !$('nativeFile').files?.length;
    $('nativeApprove').disabled = busy || uncertain() || record?.phase !== 'UPLOADING' || !record.uploadReady || !overview?.allStopped;
    $('nativeInstall').disabled = $('nativeApprove').disabled || !$('nativeApprove').checked;
    $('nativeObserve').disabled = busy || broken || !saved;
    $('nativeRetire').disabled = busy || broken || !saved || pending || saved.step === 'RETIRE_REQUESTED'
      || !!record && !['UPLOADING', 'UNOBSERVED'].includes(record.phase);
    $('nativeVerify').disabled = busy || record?.canVerify !== true || pending;
    $('nativeFinalize').disabled = busy || record?.phase !== 'APPLIED' || !record.finalizationRequired;
    $('nativeFinish').disabled = busy || !terminal || record.finalizationRequired;
      const native = $('mode').value === 'native', verified = overview?.installedCore?.state === 'VERIFIED';
      $('read').disabled = busy || native && !verified;
      $('status').disabled = busy || !verified;
      $('start').disabled = busy || broken || !verified || !!record?.finalizationRequired || !!saved && (pending || uncertain() && saved.step !== 'UPLOAD_REQUESTED');
  }
  function render() {
    for (const [key, value] of Object.entries(labels[getLanguage()])) if ($(key)) $(key).textContent = value;
      const core = overview?.installedCore;
      $('nativeInstalledState').textContent = text(!overview ? 'uncheckedCore' : core?.state === 'ABSENT' ? 'absentCore' : core?.state === 'VERIFIED' ? 'verifiedCore' : 'unverifiedCore')
        + (core?.state === 'VERIFIED' ? ` ${core.version} (${core.architecture})` : '');
      $('nativeHostState').textContent = overview ? text(overview.hostSnapshotExpired ? 'refreshHost' : !overview.packages.length ? 'empty' : overview.allStopped &&
      (!overview.maintenance || overview.maintenance.phase === 'STOPPED') ? 'readyHost' : 'busyHost') : '';
    const row = record?.package ?? selected();
    $('nativePackageInfo').textContent = row ? `${row.version} · ${row.architecture} · ${row.archiveSize} bytes · SHA-256 ${row.archiveSha256}` : '';
    const state = broken || uncertain() ? 'unknown' : record?.phase === 'APPLIED' ? record.finalizationRequired ? 'held' : 'applied'
      : record?.phase === 'RETIRED' ? 'retired' : ['ACCEPTED', 'PENDING'].includes(record?.phase) ? 'pending'
      : record?.phase === 'UPLOADING' ? record.uploadReady ? 'ready' : 'uploading' : 'idle';
    $('nativeResult').textContent = (saved ? `${saved.operationId}\n` : '') + text(state);
    $('nativeScope').textContent = overview ? `${text('scope')}: ${overview.units.length}` : '';
    buttons();
  }
  function accept(value) {
    if (!saved || value.operationId !== saved.operationId || !phases.includes(value.phase)) throw new Error('target');
    record = value;
    if (value.phase === 'APPLIED' || value.phase === 'RETIRED') remember({ ...saved, step: value.phase });
    else if (value.phase === 'UPLOADING' && (value.rejectedBeforeIntent || saved.step === 'UPLOAD_REQUESTED' && value.uploadReady)) remember({ ...saved, step: 'READY' });
    if (value.phase === 'APPLIED') onInstalled();
    render();
  }
  async function refresh() {
    overview = await invoke('native-overview', []);
    if (!Array.isArray(overview.packages) || !Array.isArray(overview.units)) throw new Error('target');
    const previous = $('nativePackage').value;
    $('nativePackage').replaceChildren();
    for (const row of overview.packages) {
      const option = document.createElement('option'); option.value = row.key; option.textContent = `${row.version} (${row.architecture})`; $('nativePackage').append(option);
    }
    if (overview.packages.some(row => row.key === previous)) $('nativePackage').value = previous;
    if (!saved && !broken && overview.current) {
      const value = overview.current;
      if (!uuid.test(value.operationId) || !value.package?.key) throw new Error('target');
      remember({ operationId: value.operationId, packageKey: value.package.key, step: 'INSTALL_REQUESTED' });
      accept(value);
    }
    render();
  }
  async function observe() {
    if (!saved) return;
    accept(await invoke('native-operation', ['--operation', saved.operationId]));
    if (record.phase === 'APPLIED') await refresh();
  }
  function schedule(left = 15, current = generation) {
    if (current !== generation || !['ACCEPTED', 'PENDING'].includes(record?.phase)) return;
    if (!left) { $('nativeResult').textContent += `\n${text('pollEnded')}`; return; }
    timer = setTimeout(() => {
      if (current !== generation) return;
      if (getBusy()) { schedule(left - 1, current); return; }
      void action(async () => { await observe(); schedule(left - 1, current); });
    }, 1000);
  }
  $('nativeRefresh').onclick = () => action(async () => { $('nativeApprove').checked = false; await refresh(); });
  $('nativeObserve').onclick = () => action(async () => { generation++; clearTimeout(timer); await observe(); await refresh(); schedule(); });
  $('nativePackage').onchange = () => { $('nativeApprove').checked = false; render(); };
  $('nativeFile').onchange = buttons;
  $('nativeApprove').onchange = buttons;
  $('nativeUpload').onclick = () => action(async () => {
    const release = selected(), file = $('nativeFile').files?.[0];
    if (saved || broken || !release || !file) throw new Error('target');
    if (file.size !== release.archiveSize || file.size > 256 * 1024 * 1024) { $('nativeResult').textContent = text('badFile'); return; }
    const data = await file.arrayBuffer();
    const hash = [...new Uint8Array(await crypto.subtle.digest('SHA-256', data))].map(x => x.toString(16).padStart(2, '0')).join('');
    if (hash !== release.archiveSha256) { $('nativeResult').textContent = text('badFile'); return; }
    remember({ operationId: crypto.randomUUID(), packageKey: release.key, step: 'UPLOAD_REQUESTED' });
    accept(await invoke('native-upload-prepare', ['--operation', saved.operationId, '--package', saved.packageKey]));
    const path = `/var/lib/yofc-console-native-packages/uploads/${saved.operationId}.deb`;
    if (record.phase !== 'UPLOADING' || record.uploadPath !== path) throw new Error('target');
    const channel = window.cockpit.file(path, { binary: true, superuser: 'require' });
    try { await channel.replace(new Uint8Array(data)); } finally { channel.close(); }
    await observe();
  });
  async function mutate(command, step) {
    if (!saved || broken) throw new Error('target');
    generation++; clearTimeout(timer); $('nativeApprove').checked = false;
    remember({ ...saved, step });
    accept(await invoke(command, ['--operation', saved.operationId]));
    if (record.phase === 'APPLIED') await refresh();
    if (record.rejectedBeforeIntent) $('nativeResult').textContent += `\n${text('rejected')}`;
    schedule();
  }
  $('nativeInstall').onclick = () => action(async () => {
    if (uncertain() || !record?.uploadReady || !$('nativeApprove').checked || !overview?.allStopped) throw new Error('target');
    await mutate('native-install', 'INSTALL_REQUESTED');
  });
  $('nativeRetire').onclick = () => action(() => mutate('native-retire', 'RETIRE_REQUESTED'));
  $('nativeVerify').onclick = () => action(() => mutate('native-verify', 'VERIFY_REQUESTED'));
  $('nativeFinalize').onclick = () => action(() => mutate('native-finalize', 'FINALIZE_REQUESTED'));
  $('nativeFinish').onclick = () => action(async () => {
    if (!record || !['APPLIED', 'RETIRED'].includes(record.phase) || record.finalizationRequired) throw new Error('target');
    localStorage.removeItem(storageKey); saved = null; record = null; $('nativeFile').value = ''; await refresh();
  });
  if (saved && !broken) setTimeout(() => { void action(observe); }, 0);
  return { render, buttons,
    serviceMayChange() { if (overview) overview = { ...overview, allStopped: false, hostSnapshotExpired: true }; render(); },
    changed() { generation++; clearTimeout(timer); render(); } };
};
