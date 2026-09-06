# Make Git choose from a set of author emails randomly

A tool that sets the current globally configured Git author email to
one chosen at random from a set of provided ones, if the configured
Git author matches the one specified to the tool.

This can be run from crontab on a user account where multiple people
have access and share dev work (co-working) and don't want to create a
new mail account for the shared work.

E.g.

    gitauthor-set-random "Yin and Yang" yin@yang.com yang@yin.com

