/*
 * Copyright (c) 2012-2014 Wind River Systems, Inc.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include <Display.h>

#include <zephyr/kernel.h>

#include <cstdio>

int main(void)
{
	printf("Hello World! %s\n", CONFIG_BOARD);

	auto disp = Display::init();
	if (!disp.has_value())
	{
		printf("Failed to initialize display.\n");
		printf("Entering busy wait loop.\n");

		while (true)
		{
		}
	}
	else
	{
		bool on = false;
		while (true)
		{
			if (on)
			{
				disp->turnBlack();
			}
			else
			{
				disp->turnWhite();
			}
			on = !on;
			k_msleep(1000);
		}

		disp->turnWhite();
	}

	return 0;
}
