#!/usr/bin/env python3

"""
Test the commit message format.

This test works properly on the local machine only when the environment
variables REMOTE and BASE_BRANCH are set. Otherwise the default values
are "origin" for the remote name of the upstream repository and "main"
for the name of the base branch, and this test may not work as expected.
"""

import os
import subprocess


COMMIT_TITLE_MAX_LEN = 60
COMMIT_BODY_LINE_MAX_LEN = 75
REMOTE = \
    os.environ.get('REMOTE') or \
    "origin"
BASE_BRANCH = \
    os.environ.get('GITHUB_HEAD_REF') or \
    os.environ.get('BASE_BRANCH') or \
    "main"


def get_cmd_output(cmd):
    """Returns stdout content of `cmd` command."""
    cmd_out = subprocess.run(cmd, shell=True, check=True,
                             stdout=subprocess.PIPE)
    stdout = cmd_out.stdout.decode('utf-8')
    return stdout


def test_commit_format():
    """
    Checks commit message format for the current PR's commits.

    Checks if commit messages follow the 60/75 commit rule (a maximum
    60 characters for the title and 75 characters for description
    lines) and if commits are signed.
    """

    # Fetch the upstream repository.
    fetch_base_cmd = "git fetch {} {}".format(REMOTE, BASE_BRANCH)
    try:
        subprocess.run(fetch_base_cmd, shell=True, check=True)
    except subprocess.CalledProcessError:
        raise NameError(
            "The name of the base branch or remote is invalid. "
            "See test documentation for more details."
        ) from None
    # Get hashes of PR's commits in their abbreviated form for
    # a prettier printing.
    shas_cmd = "git log --no-merges --pretty=%h --no-decorate " \
               f"{REMOTE}/main..{REMOTE}/{BASE_BRANCH}"
    shas = get_cmd_output(shas_cmd)
    print(shas)

    for sha in shas.split():
        # Do not enforce the commit rules when the committer is dependabot.
        author_cmd = "git show -s --format='%ae' " + sha
        author = get_cmd_output(author_cmd)

        if "dependabot" in author:
            continue
        message_cmd = "git show --pretty=format:%B -s " + sha
        message = get_cmd_output(message_cmd)
        message_lines = message.split("\n")
        print(message_lines)
        assert len(message_lines) >= 3,\
            "The commit '{}' should contain at least 3 lines: title, " \
            "blank line and a sign-off one." \
            .format(sha)
        title = message_lines[0]
        assert message_lines[1] == "",\
            "For commit '{}', title is divided into multiple lines. " \
            "Please keep it one line long and make sure you add a blank " \
            "line between title and description.".format(sha)
        assert len(title) <= COMMIT_TITLE_MAX_LEN,\
            "For commit '{}', title exceeds {} chars. " \
            "Please keep it shorter.".format(sha, COMMIT_TITLE_MAX_LEN)

        found_signed_off = False

        for line in message_lines[2:]:
            if line.startswith("Signed-off-by: "):
                found_signed_off = True
                # If we found `Signed-off-by` line, then it means
                # the commit message ended and we don't want to check
                # line lengths anymore for the current commit.
                break
            assert len(line) <= COMMIT_BODY_LINE_MAX_LEN,\
                "For commit '{}', message line '{}' exceeds {} chars. " \
                "Please keep it shorter or split it in " \
                "multiple lines.".format(sha, line,
                                         COMMIT_BODY_LINE_MAX_LEN)
        assert found_signed_off, "Commit '{}' is not signed. " \
                                 "Please run 'git commit -s --amend' " \
                                 "on it.".format(sha)
