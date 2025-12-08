import { useState } from 'react';
import { Bell, Trash2, TestTube2, Plus, Mail, MessageSquare, Send, Edit2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Switch } from '@/components/ui/switch';
import { toast } from 'sonner';
import { invoke } from '@tauri-apps/api/core';
import { NotificationChannelDialog } from './NotificationChannelDialog';
import { useTranslation } from 'react-i18next';
import { cn } from '@/lib/utils';

export interface NotificationChannelDto {
  id: string;
  channel_type: string;
  config: string;  // JSON string from Rust
  enabled: boolean;
  created_at: string;
}

interface NotificationChannelListProps {
  channels: NotificationChannelDto[];
  onUpdate: () => void;
}

export function NotificationChannelList({ channels, onUpdate }: NotificationChannelListProps) {
  const { t } = useTranslation();
  const [showDialog, setShowDialog] = useState(false);
  const [editingChannel, setEditingChannel] = useState<NotificationChannelDto | null>(null);
  const [testingId, setTestingId] = useState<string | null>(null);

  const handleDelete = async (channelId: string) => {
    try {
      await invoke('delete_notification_channel', { channelId });
      toast.success(t('notification.deleted'));
      onUpdate();
    } catch (err) {
      toast.error(t('common.error'), {
        description: String(err),
      });
    }
  };

  const handleToggle = async (channel: NotificationChannelDto) => {
    try {
      await invoke('update_notification_channel', {
        input: {
          channel_id: channel.id,
          enabled: !channel.enabled,
          config: null,
        },
      });
      toast.success(channel.enabled ? t('notification.disabled') : t('notification.enabled'));
      onUpdate();
    } catch (err) {
      toast.error(t('common.error'), {
        description: String(err),
      });
    }
  };

  const handleTest = async (channelId: string) => {
    setTestingId(channelId);
    try {
      const result = await invoke<{ success: boolean; message: string }>('test_notification_channel', {
        channelId,
      });

      if (result.success) {
        toast.success(t('notification.testSuccess'), {
          description: result.message,
        });
      } else {
        toast.error(t('notification.testFailed'), {
          description: result.message,
        });
      }
    } catch (err) {
      toast.error(t('notification.testFailed'), {
        description: String(err),
      });
    } finally {
      setTestingId(null);
    }
  };

  const handleEdit = (channel: NotificationChannelDto) => {
    setEditingChannel(channel);
    setShowDialog(true);
  };

  const handleAddNew = () => {
    setEditingChannel(null);
    setShowDialog(true);
  };

  const handleDialogClose = (success: boolean) => {
    setShowDialog(false);
    setEditingChannel(null);
    if (success) {
      onUpdate();
    }
  };

  const getChannelTypeIcon = (type: string) => {
    switch (type) {
      case 'feishu':
        return MessageSquare;
      case 'dingtalk':
        return Send;
      case 'email':
        return Mail;
      default:
        return Bell;
    }
  };

  const getChannelTypeName = (type: string) => {
    return t(`notification.channel.${type}`, type);
  };

  const getChannelStyle = (type: string) => {
    switch (type) {
      case 'feishu':
        return 'bg-[#3370ff]/10 text-[#3370ff] dark:bg-[#3370ff]/20 dark:text-[#5c8dff]';
      case 'dingtalk':
        return 'bg-[#007fff]/10 text-[#007fff] dark:bg-[#007fff]/20 dark:text-[#4da6ff]';
      case 'email':
        return 'bg-purple-500/10 text-purple-600 dark:bg-purple-500/20 dark:text-purple-400';
      default:
        return 'bg-gray-500/10 text-gray-600 dark:bg-gray-500/20 dark:text-gray-400';
    }
  };

  const parseConfig = (configStr: string) => {
    try {
      return JSON.parse(configStr);
    } catch {
      return {};
    }
  };

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between pb-2">
        <Button onClick={handleAddNew} className="rounded-full shadow-sm ml-auto">
          <Plus className="h-4 w-4 mr-2" />
          {t('notification.addChannel')}
        </Button>
      </div>

      {channels.length === 0 ? (
        <Card className="rounded-2xl border-2 border-dashed">
          <CardContent className="py-16 text-center">
            <div className="w-20 h-20 mx-auto mb-6 rounded-full bg-primary/10 flex items-center justify-center">
              <Bell className="h-10 w-10 text-primary" />
            </div>
            <h3 className="text-lg font-semibold mb-2">{t('notification.noChannels')}</h3>
            <p className="text-muted-foreground mb-6 max-w-md mx-auto">
              {t('notification.noChannelsDesc')}
            </p>
            <Button onClick={handleAddNew} size="lg" className="rounded-full">
              <Plus className="h-5 w-5 mr-2" />
              {t('notification.addFirstChannel')}
            </Button>
          </CardContent>
        </Card>
      ) : (
        <div className="space-y-3">
          {channels.map((channel) => {
            const config = parseConfig(channel.config);
            const Icon = getChannelTypeIcon(channel.channel_type);
            const styleClass = getChannelStyle(channel.channel_type);
            
            return (
              <Card 
                key={channel.id} 
                className={cn(
                  "border shadow-sm transition-all duration-200 overflow-hidden",
                  channel.enabled ? "bg-card" : "bg-muted/30 opacity-90"
                )}
              >
                <CardContent className="p-4 flex flex-col md:flex-row md:items-center gap-4">
                  {/* Left: Icon & Main Info */}
                  <div className="flex items-start gap-4 flex-1 min-w-0">
                    <div className={cn(
                      "w-12 h-12 rounded-xl flex items-center justify-center shrink-0 mt-1 md:mt-0",
                      styleClass
                    )}>
                      <Icon className="h-6 w-6" />
                    </div>

                    <div className="flex-1 min-w-0 space-y-1">
                      <div className="flex items-center gap-2">
                        <h4 className="font-semibold text-base">
                          {getChannelTypeName(channel.channel_type)}
                        </h4>
                        {!channel.enabled && (
                          <Badge variant="secondary" className="text-[10px] h-5 px-1.5">
                            {t('notification.disabled')}
                          </Badge>
                        )}
                      </div>

                      <div className="text-sm text-muted-foreground font-mono truncate max-w-md">
                        {channel.channel_type === 'feishu' && config.webhook_key && (
                          <span className="flex items-center gap-2">
                            <span className="text-xs uppercase tracking-wider opacity-70">Webhook</span>
                            {config.webhook_key}
                          </span>
                        )}
                        {channel.channel_type === 'dingtalk' && config.webhook_key && (
                          <span className="flex items-center gap-2">
                            <span className="text-xs uppercase tracking-wider opacity-70">Token</span>
                            {config.webhook_key}
                          </span>
                        )}
                        {channel.channel_type === 'email' && config.from && (
                          <span className="flex items-center gap-2">
                            <span className="text-xs uppercase tracking-wider opacity-70">From</span>
                            {config.from}
                          </span>
                        )}
                      </div>
                    </div>
                  </div>

                  {/* Right: Actions */}
                  <div className="flex items-center justify-between md:justify-end gap-3 md:gap-4 pt-2 md:pt-0 border-t md:border-t-0 border-border/50">
                     <div className="flex items-center gap-2 mr-2">
                        <span className="text-sm text-muted-foreground hidden lg:inline-block">
                          {channel.enabled ? t('common.active') : t('common.disabled')}
                        </span>
                        <Switch
                          checked={channel.enabled}
                          onCheckedChange={() => handleToggle(channel)}
                        />
                     </div>
                     
                     <div className="h-8 w-px bg-border hidden md:block" />

                     <div className="flex items-center gap-2">
                       <Button
                         variant="ghost"
                         size="sm"
                         onClick={() => handleTest(channel.id)}
                         disabled={testingId === channel.id}
                         className="h-8 px-2 lg:px-3 text-muted-foreground hover:text-foreground"
                       >
                         {testingId === channel.id ? (
                            <span className="animate-spin mr-2">⟳</span>
                         ) : (
                            <TestTube2 className="h-4 w-4 mr-2" />
                         )}
                         {t('notification.test')}
                       </Button>

                       <Button
                         variant="ghost"
                         size="sm"
                         onClick={() => handleEdit(channel)}
                         className="h-8 px-2 lg:px-3 text-muted-foreground hover:text-foreground"
                       >
                         <Edit2 className="h-4 w-4 mr-2" />
                         {t('common.edit')}
                       </Button>

                       <Button
                         variant="ghost"
                         size="sm"
                         onClick={() => handleDelete(channel.id)}
                         className="h-8 px-2 lg:px-3 text-destructive hover:text-destructive hover:bg-destructive/10"
                       >
                         <Trash2 className="h-4 w-4 mr-2" />
                         {t('common.delete')}
                       </Button>
                     </div>
                  </div>
                </CardContent>
              </Card>
            );
          })}
        </div>
      )}

      <NotificationChannelDialog
        open={showDialog}
        onClose={handleDialogClose}
        channel={editingChannel}
      />
    </div>
  );
}
