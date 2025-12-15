import { Resend } from 'resend';
import { env } from '$env/dynamic/private';

const resend = new Resend(env.RESEND_API_KEY);

export async function sendMagicLinkEmail(email: string, token: string): Promise<boolean> {
	const baseUrl = env.DEV_URL || env.PROD_URL || 'http://localhost:5173';
	const magicLink = `${baseUrl}/auth/verify?email=${encodeURIComponent(email)}&token=${token}`;

	try {
		const { error } = await resend.emails.send({
			from: env.FROM_EMAIL || 'support@geni.health',
			to: email,
			subject: 'Access Your Geni Report',
			html: `
				<!DOCTYPE html>
				<html>
				<head>
					<meta charset="utf-8">
					<meta name="viewport" content="width=device-width, initial-scale=1.0">
				</head>
				<body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 0; background-color: #f5f5f5;">
					<table width="100%" cellpadding="0" cellspacing="0" style="max-width: 500px; margin: 0 auto; padding: 40px 20px;">
						<tr>
							<td style="background-color: #ffffff; border-radius: 16px; padding: 40px; text-align: center;">
								<h1 style="color: #1a1a1a; font-size: 24px; margin: 0 0 16px 0;">Access Your Report</h1>
								<p style="color: #666666; font-size: 16px; line-height: 1.5; margin: 0 0 24px 0;">
									Click the button below to access your DNA insights report. This link expires in 15 minutes.
								</p>
								<a href="${magicLink}" style="display: inline-block; background-color: #e85d04; color: #ffffff; text-decoration: none; padding: 14px 32px; border-radius: 8px; font-size: 16px; font-weight: 500;">
									View My Report
								</a>
								<p style="color: #999999; font-size: 14px; line-height: 1.5; margin: 32px 0 0 0;">
									If you didn't request this email, you can safely ignore it.
								</p>
							</td>
						</tr>
						<tr>
							<td style="text-align: center; padding: 24px 0;">
								<p style="color: #999999; font-size: 12px; margin: 0;">
									Geni - Educational DNA Insights<br>
									This is not medical advice.
								</p>
							</td>
						</tr>
					</table>
				</body>
				</html>
			`
		});

		if (error) {
			console.error('Failed to send magic link email:', error);
			return false;
		}

		return true;
	} catch (error) {
		console.error('Error sending magic link email:', error);
		return false;
	}
}
