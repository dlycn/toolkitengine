startBtn.addEventListener('click', function() {
    isActive = !isActive;
    if (isActive) {
        renderAbc();
        startBtn.classList.add('active');
    } else {
        startBtn.classList.remove('active');
    }
});

saveBtn.addEventListener('click', function() {
    if (isSaving) return;
    saveBtn.classList.add('active');
    isSaving = true;

    let abcString = abcCode.value.split("\n *").join("\n");
    if (abcString.trim().length === 0) {
        alert('The lyrics is empty.');
        saveBtn.classList.remove('active');
        isSaving = false;
        return;
    }
        fetch('/save', {
            method: 'POST',
            headers: {
                'Content-Type': 'text/plain'
            },
            body: abcString
        })
        .then(response => {
            if (response.ok) {
                alert('CODE saved successfully.');
            } else {
                alert('Failed to save CODE.');
            }
        })
        .catch(error => {
            console.error('Error:', error);
            alert('CODE save error.');
        })
        .finally(() => {
            saveBtn.classList.remove('active');
            isSaving = false;
        });
});

// 播放按钮点击：切换播放/停止
playBtn.addEventListener('click', function() {
    console.log('playBtn clicked');
    togglePlay();
});

abcCode.addEventListener('input', function() {
    if (!isActive) return;
    renderAbc();
});